use std::sync::Arc;
use std::sync::Mutex;
use fragile::Fragile;
use futures::channel::{mpsc, oneshot};
use futures::future::Future;
use futures::stream::Stream;
use futures::task;
use futures::task::Poll;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use std::marker::Unpin;
use std::pin;
use std::pin::Pin;

use glib;

/// Represents a `Future` around a `glib::Source`. The future will
/// be resolved once the source has provided a value
pub struct GLibReceiverFuture<T> {
    receiver: Option<glib::Receiver<T>>,
    source: Option<(glib::SourceId, oneshot::Receiver<Fragile<T>>)>,
}

impl<T> GLibReceiverFuture<T>
{
    /// Create a new `GLibReceiverFuture`
    ///
    /// The provided closure should return a newly created `glib::Source` when called
    /// and pass the value provided by the source to the oneshot sender that is passed
    /// to the closure.
    pub fn new(receiver: glib::Receiver<T>) -> GLibReceiverFuture<T> {
        GLibReceiverFuture {
            receiver: Some(receiver),
            source: None,
        }
    }
}

impl<T> Unpin for GLibReceiverFuture<T> {}

impl<T> Future for GLibReceiverFuture<T>
{
    type Output = T;

    fn poll(mut self: pin::Pin<&mut Self>, ctx: &mut task::Context) -> Poll<T> {

        let GLibReceiverFuture {
            ref mut receiver,
            ref mut source,
            ..
        } = *self;

        if let Some(receiver) = receiver.take() {

            let main_context = glib::MainContext::ref_thread_default();
            assert!(
                main_context.is_owner(),
                "Spawning futures only allowed if the thread is owning the MainContext"
            );

            // Channel for sending back the Source result to our future here.
            //
            // In theory we could directly continue polling the
            // corresponding task from the Source callback,
            // however this would break at the very least
            // the g_main_current_source() API.
            let (send, recv) = oneshot::channel();

            let source_id = receiver.attach(Some(&main_context), move |val| {
                let _ = send.send(Fragile::new(val));
                glib::Continue(false)
            });

            *source = Some((source_id, recv));
        }

        // At this point we must have a receiver
        let res = {
            let &mut (_, ref mut receiver) = source.as_mut().unwrap();
            receiver.poll_unpin(ctx)
        };
        #[allow(clippy::match_wild_err_arm)]
        match res {
            Poll::Ready(Err(_)) => panic!("Source sender was unexpectedly closed"),
            Poll::Ready(Ok(v)) => {
                // Get rid of the reference to the source, it triggered
                // let _ = source.take().lock();
                Poll::Ready(v.into_inner())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// impl<T> Drop for GLibReceiverFuture<T> {
//     fn drop(&mut self) {
//         // Get rid of the source, we don't care anymore if it still triggers
//         if let Some((source, _)) = self.source.take() {
//             source.destroy();
//         }
//     }
// }
