use std::{
    fmt,
    hash::Hash,
    io,
    marker::PhantomData,
    pin::Pin,
    time::{Duration, SystemTime},
};
use tarpc::{
    context, ClientMessage, Request, Response,
    ServerError, Transport,
};
use tarpc::rpc::util::TimeUntil;
use tarpc::server::{Serve, Channel, Resp, RespState, RequestHandler};

use futures::{
    channel::mpsc,
    future::{Abortable},
    prelude::*,
    ready,
    stream::Fuse,
    task::*,
};
use humantime::format_rfc3339;
use log::*;
use pin_project::pin_project;
use tokio;


use glib;

pub(crate) type PollIo<T> = Poll<Option<io::Result<T>>>;


/// A running handler serving all requests coming over a channel.
#[pin_project]
#[derive(Debug)]
pub struct ClientHandler<C, S>
where
    C: Channel,
{
    #[pin]
    channel: C,
    /// Responses waiting to be written to the wire.
    #[pin]
    pending_responses: Fuse<mpsc::Receiver<(context::Context, Response<C::Resp>)>>,
    /// Handed out to request handlers to fan in responses.
    #[pin]
    responses_tx: mpsc::Sender<(context::Context, Response<C::Resp>)>,
    /// Server
    server: S,
}

impl<C, S> ClientHandler<C, S>
where
    C: Channel + 'static,
    C::Req: Send + 'static,
    C::Resp: Send + 'static,
    S: Serve<C::Req, Resp = C::Resp> + Send + 'static,
    S::Fut: Send + 'static,
{
    /// Runs the client handler until completion by spawning each
    /// request handler onto the default executor.
    pub fn execute(self) -> impl Future<Output = ()> {
        use log::info;

        self.try_for_each(|request_handler| async {
            let ctx = glib::MainContext::ref_thread_default();
            ctx.spawn(request_handler);
            Ok(())
        })
        .unwrap_or_else(|e| info!("ClientHandler errored out: {}", e))
    }
}

impl<C, S> ClientHandler<C, S>
where
    C: Channel,
    S: Serve<C::Req, Resp = C::Resp>,
{
    fn pump_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> PollIo<RequestHandler<S::Fut, C::Resp>> {
        match ready!(self.as_mut().project().channel.poll_next(cx)?) {
            Some(request) => Poll::Ready(Some(Ok(self.handle_request(request)))),
            None => Poll::Ready(None),
        }
    }

    fn pump_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        read_half_closed: bool,
    ) -> PollIo<()> {
        match self.as_mut().poll_next_response(cx)? {
            Poll::Ready(Some((ctx, response))) => {
                trace!(
                    "[{}] Staging response. In-flight requests = {}.",
                    ctx.trace_id(),
                    self.as_mut().project().channel.in_flight_requests(),
                );
                self.as_mut().project().channel.start_send(response)?;
                Poll::Ready(Some(Ok(())))
            }
            Poll::Ready(None) => {
                // Shutdown can't be done before we finish pumping out remaining responses.
                ready!(self.as_mut().project().channel.poll_flush(cx)?);
                Poll::Ready(None)
            }
            Poll::Pending => {
                // No more requests to process, so flush any requests buffered in the transport.
                ready!(self.as_mut().project().channel.poll_flush(cx)?);

                // Being here means there are no staged requests and all written responses are
                // fully flushed. So, if the read half is closed and there are no in-flight
                // requests, then we can close the write half.
                if read_half_closed && self.as_mut().project().channel.in_flight_requests() == 0 {
                    Poll::Ready(None)
                } else {
                    Poll::Pending
                }
            }
        }
    }

    fn poll_next_response(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> PollIo<(context::Context, Response<C::Resp>)> {
        // Ensure there's room to write a response.
        while let Poll::Pending = self.as_mut().project().channel.poll_ready(cx)? {
            ready!(self.as_mut().project().channel.poll_flush(cx)?);
        }

        match ready!(self.as_mut().project().pending_responses.poll_next(cx)) {
            Some((ctx, response)) => Poll::Ready(Some(Ok((ctx, response)))),
            None => {
                // This branch likely won't happen, since the ClientHandler is holding a Sender.
                Poll::Ready(None)
            }
        }
    }

    fn handle_request(
        mut self: Pin<&mut Self>,
        request: Request<C::Req>,
    ) -> RequestHandler<S::Fut, C::Resp> {
        let request_id = request.id;
        let deadline = request.context.deadline;
        let timeout = deadline.time_until();
        trace!(
            "[{}] Received request with deadline {} (timeout {:?}).",
            request.context.trace_id(),
            format_rfc3339(deadline),
            timeout,
        );
        let ctx = request.context;
        let request = request.message;

        let response = self.as_mut().project().server.clone().serve(ctx, request);
        let response = Resp {
            state: RespState::PollResp,
            request_id,
            ctx,
            deadline,
            f: tokio::time::timeout(timeout, response),
            response: None,
            response_tx: self.as_mut().project().responses_tx.clone(),
        };
        let abort_registration = self.as_mut().project().channel.start_request(request_id);
        RequestHandler {
            resp: Abortable::new(response, abort_registration),
        }
    }
}

impl<C, S> Stream for ClientHandler<C, S>
where
    C: Channel,
    S: Serve<C::Req, Resp = C::Resp>,
{
    type Item = io::Result<RequestHandler<S::Fut, C::Resp>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let read = self.as_mut().pump_read(cx)?;
            let read_closed = if let Poll::Ready(None) = read {
                true
            } else {
                false
            };
            match (read, self.as_mut().pump_write(cx, read_closed)?) {
                (Poll::Ready(None), Poll::Ready(None)) => {
                    return Poll::Ready(None);
                }
                (Poll::Ready(Some(request_handler)), _) => {
                    return Poll::Ready(Some(Ok(request_handler)));
                }
                (_, Poll::Ready(Some(()))) => {}
                _ => {
                    return Poll::Pending;
                }
            }
        }
    }
}



/// A future that drives the server by spawning channels and request handlers on the default
/// executor.
#[pin_project]
#[derive(Debug)]
pub struct Running<St, Se> {
    #[pin]
    incoming: St,
    server: Se,
}

impl<St, C, Se> Future for Running<St, Se>
where
    St: Sized + Stream<Item = C>,
    C: Channel + Send + 'static,
    C::Req: Send + 'static,
    C::Resp: Send + 'static,
    Se: Serve<C::Req, Resp = C::Resp> + Send + 'static + Clone,
    Se::Fut: Send + 'static,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        use log::info;

        while let Some(channel) = ready!(self.as_mut().project().incoming.poll_next(cx)) {
            let ctx = glib::MainContext::ref_thread_default();
            ctx.spawn(
                clienthandler_respond_with(channel, self.as_mut().project().server.clone())
                .execute()
            );
        }
        info!("Server shutting down.");
        Poll::Ready(())
    }
}

/// Respond to requests coming over the channel with `f`. Returns a future that drives the
/// responses and resolves when the connection is closed.
pub fn clienthandler_respond_with<C, S>(channel: C, server: S) -> ClientHandler<C, S>
where
    C: Transport<Response<<C as Channel>::Resp>, Request<<C as Channel>::Req>> + Channel + Sized,
    S: Serve<<C as Channel>::Req, Resp = <C as Channel>::Resp>
{
    let (responses_tx, responses) = mpsc::channel(channel.config().pending_response_buffer);
    let responses = responses.fuse();

    ClientHandler {
        channel,
        server,
        pending_responses: responses,
        responses_tx,
    }
}

/// Responds to all requests with `server`.
pub fn handler_respond_with<H, C, S>(handler: H, server: S) -> Running<H, S>
where
    S: Serve<C::Req, Resp = C::Resp>,
    H: Sized + Stream<Item = C>,
    C: Channel,
{
    Running {
        incoming: handler,
        server,
    }
}


