use async_std::io::Error;
use async_trait::*;
use bincode;
use bytes::{Buf, BytesMut, IntoBuf};
use futures::task::Context;
use futures::task::Poll;
use futures::AsyncReadExt;
use futures::AsyncWriteExt;
use futures::Sink;
use futures::Stream;
use futures::TryFuture;
use futures::TryFutureExt;
use futures::{pin_mut, ready, Future};
use futures::{AsyncRead, AsyncWrite};
use gio;
use gio::prelude::*;
use pin_project::pin_project;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::pin::Pin;
use std::rc::Rc;

// TODO: replace all of this with a generic implementation that wraps AsyncRead and AsyncWrite

pub fn connection<T>(
    socket: gio::Socket,
) -> Result<Connection<T, gio::IOStreamAsyncReadWrite<gio::SocketConnection>>, Error>
where
    T: for<'de> Deserialize<'de> + Serialize + Send,
{
    let connection = socket.connection_factory_create_connection();
    let stream = connection.into_async_read_write().unwrap();

    Ok(Connection {
        connection: stream,
        marker: PhantomData,
        buffer: BytesMut::new(),
    })
}

/// Low-water mark
const LW: usize = 1024;
/// High-water mark
const HW: usize = 8 * 1024;

#[derive(Debug)]
#[pin_project]
pub struct Sender<T, W>
where
    T: Serialize,
    W: AsyncWrite + fmt::Debug,
{
    #[pin]
    writer: W,
    marker: PhantomData<T>,
    buffer: BytesMut,
}

#[derive(Debug)]
#[pin_project]
pub struct Receiver<T, R>
where
    T: for<'de> Deserialize<'de>,
    R: AsyncRead + fmt::Debug,
{
    #[pin]
    reader: R,
    marker: PhantomData<T>,
}

impl<T, W> Sink<T> for Sender<T, W>
where
    T: Serialize,
    W: AsyncWrite + AsyncWriteExt + fmt::Debug,
{
    type Error = std::io::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();
        let writer: Pin<&mut W> = this.writer;
        writer.poll_flush(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let mut this = self.project();
        let remaining = this.buffer.capacity() - this.buffer.len();
        if remaining < LW {
            this.buffer.reserve(HW - remaining);
        }

        // TODO: process err
        *this.buffer = bincode::serialize(&item).unwrap().into();
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        dbg!("flushing framed transport");
        // let writer: Pin<&mut W> = this.writer.as_mut();

        while !this.buffer.is_empty() {
            dbg!("writing; remaining={}", this.buffer.len());

            let f = this.writer.as_mut().poll_write(cx, this.buffer);
            let n = ready!(f)?;

            if n == 0 {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write frame to transport",
                )
                .into()));
            }

            // remove written data
            this.buffer.advance(n);
        }

        // Try flushing the underlying IO
        ready!(this.writer.as_mut().poll_flush(cx))?;

        dbg!("transport flushed");
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();
        let writer: Pin<&mut W> = this.writer;
        writer.poll_close(cx)
    }
}

impl<T, R> Stream for Receiver<T, R>
where
    T: for<'de> Deserialize<'de>,
    R: AsyncRead + fmt::Debug,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        let reader: Pin<&mut R> = this.reader;

        let mut buffer = Vec::with_capacity(4096);
        let res = reader.poll_read(cx, &mut buffer);

        match res {
            Poll::Ready(result) => {
                //TODO: test if result is ok and contains correct num bytes
                let val = bincode::deserialize::<T>(&buffer).unwrap();
                Poll::Ready(Some(val))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug)]
#[pin_project]
pub struct Connection<T, S>
where
    T: for<'de> Deserialize<'de> + Serialize,
    S: AsyncReadExt + AsyncWriteExt + fmt::Debug,
{
    #[pin]
    connection: S,
    marker: PhantomData<T>,
    buffer: BytesMut,
}

impl<T, S> Sink<T> for Connection<T, S>
where
    T: for<'de> Deserialize<'de> + Serialize,
    S: AsyncReadExt + AsyncWriteExt + fmt::Debug,
{
    type Error = std::io::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();
        let connection: Pin<&mut S> = this.connection;
        connection.poll_flush(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let mut this = self.project();
        let remaining = this.buffer.capacity() - this.buffer.len();
        if remaining < LW {
            this.buffer.reserve(HW - remaining);
        }

        // TODO: process err
        *this.buffer = bincode::serialize(&item).unwrap().into();
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        dbg!("flushing framed transport");

        while !this.buffer.is_empty() {
            dbg!("writing; remaining={}", this.buffer.len());

            let n = ready!(this.connection.as_mut().poll_write(cx, this.buffer))?;

            if n == 0 {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write frame to transport",
                )
                .into()));
            }

            // remove written data
            this.buffer.advance(n);
        }

        // Try flushing the underlying IO
        ready!(this.connection.as_mut().poll_flush(cx))?;

        dbg!("transport flushed");
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();
        let connection: Pin<&mut S> = this.connection;
        connection.poll_close(cx)
    }
}

impl<T, S> Stream for Connection<T, S>
where
    T: for<'de> Deserialize<'de> + Serialize,
    S: AsyncReadExt + AsyncWriteExt + fmt::Debug,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        let connection: Pin<&mut S> = this.connection;

        let mut buffer = Vec::with_capacity(4096);
        let res = connection.poll_read(cx, &mut buffer);

        match res {
            Poll::Ready(result) => {
                //TODO: test if result is ok and contains correct num bytes
                let val = bincode::deserialize::<T>(&buffer).unwrap();
                Poll::Ready(Some(val))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
