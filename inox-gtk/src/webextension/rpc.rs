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
    client::{Config, NewClient},
    server::{Serve, Channel, ClientHandler}
};

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

use glib;

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
            ctx.block_on(
                channel.respond_with(self.as_mut().project().server.clone())
                .execute()
            );
        }
        info!("Server shutting down.");
        Poll::Ready(())
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

pub fn spawn_client<C, D>(client: NewClient<C, D>) -> io::Result<C>
where
    D: Future<Output = io::Result<()>> + Send + 'static,
{
    let dispatch = client
        .dispatch
        .unwrap_or_else(move |e| error!("Connection broken: {}", e));

    let ctx = glib::MainContext::default();
    tokio::spawn(dispatch);

    Ok(client.client)
}


