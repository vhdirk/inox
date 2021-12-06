use async_std::io::Error;
use async_trait::*;
use bincode;
use bytes::{Buf, BufMut, BytesMut};
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
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::rc::Rc;
use tokio::io::ReadBuf;

/// Low-water mark
const LW: usize = 1024;
/// High-water mark
const HW: usize = 8 * 1024;

pub fn connection<T>(
    socket: gio::Socket,
) -> Result<Connection<T, gio::IOStreamAsyncReadWrite<gio::SocketConnection>>, Error>
where
    T: for<'de> Deserialize<'de> + Serialize + Send,
{
    let connection = socket.connection_factory_create_connection();
    let stream = connection.into_async_read_write().unwrap();

    Ok(Connection::new(stream))
}

// pub fn connection<T>(
//     socket: gio::Socket,
// ) -> Result<Connection<T, gio::IOStreamAsyncReadWrite<gio::SocketConnection>>, Error>
// where
//     T: for<'de> Deserialize<'de> + Serialize + Send,
// {
//     let connection = socket.connection_factory_create_connection();
//     let stream = connection.into_async_read_write().unwrap();

//     Ok(Connection {
//         connection: stream,
//         marker: PhantomData,
//         buffer: BytesMut::with_capacity(LW),
//     })
// }

// #[derive(Debug)]
// #[pin_project]
// pub struct Sender<T, W>
// where
//     T: Serialize,
//     W: AsyncWrite,
// {
//     #[pin]
//     writer: W,
//     marker: PhantomData<T>,
//     buffer: BytesMut,
// }

// #[derive(Debug)]
// #[pin_project]
// pub struct Receiver<T, R>
// where
//     T: for<'de> Deserialize<'de>,
//     R: AsyncRead,
// {
//     #[pin]
//     reader: R,
//     marker: PhantomData<T>,
// }

// impl<T, W> Sink<T> for Sender<T, W>
// where
//     T: Serialize,
//     W: AsyncWrite + Write,
// {
//     type Error = std::io::Error;

//     fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         let this = self.project();
//         let writer: Pin<&mut W> = this.writer;
//         writer.poll_flush(cx)
//     }

//     fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
//         let this = self.project();
//         let remaining = this.buffer.capacity() - this.buffer.len();
//         if remaining < LW {
//             this.buffer.reserve(HW - remaining);
//         }

//         // TODO: process err
//         debug!("Serialize buffer");

//         *this.buffer = bincode::serialize(&item).unwrap().into();
//         Ok(())
//     }

//     fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         let mut this = self.project();

//         debug!("flushing framed transport");
//         // let writer: Pin<&mut W> = this.writer.as_mut();

//         while !this.buffer.is_empty() {
//             debug!("writing; remaining={}", this.buffer.len());

//             let f = this.writer.as_mut().poll_write(cx, this.buffer);
//             let n = ready!(f)?;

//             if n == 0 {
//                 return Poll::Ready(Err(io::Error::new(
//                     io::ErrorKind::WriteZero,
//                     "failed to write frame to transport",
//                 )));
//             }

//             // remove written data
//             this.buffer.advance(n);
//         }

//         // Try flushing the underlying IO
//         ready!(this.writer.as_mut().poll_flush(cx))?;

//         debug!("transport flushed");
//         Poll::Ready(Ok(()))
//     }

//     fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         let mut this = self.project();
//         let writer: Pin<&mut W> = this.writer;
//         writer.poll_close(cx)
//     }
// }

// impl<T, R> Stream for Receiver<T, R>
// where
//     T: for<'de> Deserialize<'de>,
//     R: AsyncRead,
// {
//     type Item = T;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         let this = self.project();
//         let reader: Pin<&mut R> = this.reader;

//         let mut buffer = Vec::with_capacity(4096);
//         let res = reader.poll_read(cx, &mut buffer);

//         match res {
//             Poll::Ready(result) => {
//                 //TODO: test if result is ok and contains correct num bytes
//                 let val = bincode::deserialize::<T>(&buffer).unwrap();
//                 Poll::Ready(Some(val))
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

// #[derive(Debug)]
// #[pin_project]
// pub struct Connection<T, S>
// where
//     T: for<'de> Deserialize<'de> + Serialize,
//     S: Read + Write,
// {
//     #[pin]
//     socket: S,
//     marker: PhantomData<T>,
//     buffer: BytesMut,
// }

// impl<T, S> Connection<T, S>
// where
//     T: for<'de> Deserialize<'de> + Serialize,
//     S: Read + Write,
// {
//     pub fn new(socket: S) -> Self {
//         Self {
//             socket,
//             marker: PhantomData,
//             buffer: BytesMut::with_capacity(LW),
//         }
//     }
// }

// impl<T, S> Sink<T> for Connection<T, S>
// where
//     T: for<'de> Deserialize<'de> + Serialize,
//     S: Read + Write,
// {
//     type Error = std::io::Error;

//     fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         debug!("poll_ready");

//         let this = self.project();
//         let socket: Pin<&mut S> = this.socket;
//         socket.poll_flush(cx)
//     }

//     fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
//         debug!("start_send");

//         let this = self.project();
//         let remaining = this.buffer.capacity() - this.buffer.len();
//         if remaining < LW {
//             this.buffer.reserve(HW - remaining);
//         }

//         // TODO: process err
//         *this.buffer = bincode::serialize(&item).unwrap().into();
//         Ok(())
//     }

//     fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         debug!("poll_flush");
//         let mut this = self.project();

//         while !this.buffer.is_empty() {
//             debug!("writing; remaining={}", this.buffer.len());

//             let n = ready!(this.socket.as_mut().poll_write(cx, this.buffer))?;

//             if n == 0 {
//                 return Poll::Ready(Err(io::Error::new(
//                     io::ErrorKind::WriteZero,
//                     "failed to write frame to transport",
//                 )));
//             }

//             // remove written data
//             this.buffer.advance(n);
//         }

//         // Try flushing the underlying IO
//         ready!(this.socket.as_mut().poll_flush(cx))?;

//         debug!("transport flushed");
//         Poll::Ready(Ok(()))
//     }

//     fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         debug!("poll_close");
//         let this = self.project();
//         let socket: Pin<&mut S> = this.socket;
//         socket.poll_close(cx)
//     }
// }

// impl<T, S> Stream for Connection<T, S>
// where
//     T: for<'de> Deserialize<'de> + Serialize,
//     S: Read + Write,
// {
//     type Item = T;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         debug!("poll_next");

//         let this = self.project();
//         let socket: Pin<&mut S> = this.socket;

//         let mut buffer = Vec::with_capacity(4096);
//         let res = socket.poll_read(cx, &mut buffer);
//         debug!("Received bytes, {:?} {:?}", &res, &buffer);

//         match res {
//             Poll::Ready(result) => {
//                 if let Ok(num) = result {
//                     if num == 0 {
//                         return Poll::Pending;
//                     }

//                     //TODO: test if result is ok and contains correct num bytes
//                     debug!("Deserialize buffer, {:?}", &buffer);
//                     let val = bincode::deserialize::<T>(&buffer).unwrap();
//                     Poll::Ready(Some(val))
//                 } else {
//                     Poll::Ready(None)
//                 }
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

pub fn poll_read_buf<T: AsyncRead, B: BufMut>(
    io: Pin<&mut T>,
    cx: &mut Context<'_>,
    buf: &mut B,
) -> Poll<io::Result<usize>> {
    if !buf.has_remaining_mut() {
        return Poll::Ready(Ok(0));
    }

    let n = {
        let dst = buf.chunk_mut();
        let dst = unsafe { &mut *(dst as *mut _ as *mut [MaybeUninit<u8>]) };
        let mut buf = ReadBuf::uninit(dst);
        let ptr = buf.filled().as_ptr();

        ready!(io.poll_read(cx, buf.initialized_mut())?);

        // Ensure the pointer does not change from under us
        assert_eq!(ptr, buf.filled().as_ptr());
        buf.filled().len()
    };

    // Safety: This is guaranteed to be the number of initialized (and read)
    // bytes due to the invariants provided by `ReadBuf::filled`.
    unsafe {
        buf.advance_mut(n);
    }

    Poll::Ready(Ok(n))
}

// pub fn poll_write_buf<T: AsyncWrite, B: Buf>(
//     io: Pin<&mut T>,
//     cx: &mut Context<'_>,
//     buf: &mut B,
// ) -> Poll<io::Result<usize>> {
//     const MAX_BUFS: usize = 64;

//     if !buf.has_remaining() {
//         return Poll::Ready(Ok(0));
//     }

//     let n = if io.is_write_vectored() {
//         let mut slices = [IoSlice::new(&[]); MAX_BUFS];
//         let cnt = buf.chunks_vectored(&mut slices);
//         ready!(io.poll_write_vectored(cx, &slices[..cnt]))?
//     } else {
//         ready!(io.poll_write(cx, buf.chunk()))?
//     };

//     buf.advance(n);

//     Poll::Ready(Ok(n))
// }

#[derive(Debug)]
#[pin_project]
pub struct Connection<T, S>
where
    T: for<'de> Deserialize<'de> + Serialize + Sized,
    S: AsyncReadExt + AsyncWriteExt,
{
    #[pin]
    connection: S,
    marker: PhantomData<T>,
    write_buffer: BytesMut,
    read_buffer: BytesMut,
}

impl<T, S> Connection<T, S>
where
    T: for<'de> Deserialize<'de> + Serialize,
    S: AsyncReadExt + AsyncWriteExt,
{
    pub fn new(connection: S) -> Self {
        Self {
            connection,
            marker: PhantomData,
            write_buffer: BytesMut::with_capacity(LW),
            read_buffer: BytesMut::with_capacity(LW),
        }
    }
}

impl<T> Connection<T, gio::IOStreamAsyncReadWrite<gio::SocketConnection>>
where
    T: for<'de> Deserialize<'de> + Serialize
{
    pub fn close(&self) -> Result<(), glib::error::Error>{
        self.connection.io_stream().clear_pending();
        self.connection.io_stream().close(None::<&gio::Cancellable>)
    }
}

impl<T, S> Sink<T> for Connection<T, S>
where
    T: for<'de> Deserialize<'de> + Serialize + Sized,
    S: AsyncReadExt + AsyncWriteExt,
{
    type Error = std::io::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        let connection: Pin<&mut S> = this.connection;
        connection.poll_flush(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let this = self.project();
        let remaining = this.write_buffer.capacity() - this.write_buffer.len();
        if remaining < LW {
            this.write_buffer.reserve(HW - remaining);
        }
        this.write_buffer
            .put(bincode::serialize(&item).unwrap().as_ref());
        // TODO: process err
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        while !this.write_buffer.is_empty() {
            let n = ready!(this.connection.as_mut().poll_write(cx, this.write_buffer))?;

            if n == 0 {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write buffer to socket",
                )));
            }

            // remove written data
            this.write_buffer.advance(n);
        }

        // Try flushing the underlying IO
        ready!(this.connection.as_mut().poll_flush(cx))?;
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        let connection: Pin<&mut S> = this.connection;
        connection.poll_close(cx)
    }
}

impl<T, S> Stream for Connection<T, S>
where
    T: for<'de> Deserialize<'de> + Serialize + Sized,
    S: AsyncReadExt + AsyncWriteExt,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let connection: Pin<&mut S> = this.connection;

        unsafe {
            this.read_buffer.set_len(this.read_buffer.capacity());
        }

        let res = connection.poll_read(cx, this.read_buffer);

        match res {
            Poll::Ready(result) => {
                if let Ok(num) = result {
                    if num == 0 {
                        return Poll::Pending;
                    }

                    //TODO: test if result is ok and contains correct num bytes
                    let val = bincode::deserialize::<T>(this.read_buffer).unwrap();
                    this.read_buffer.advance(num);

                    Poll::Ready(Some(val))
                } else {
                    Poll::Ready(None)
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

mod tests {
    use super::*;
    use bytes::BytesMut;
    use futures::io::{AsyncReadExt, AsyncWriteExt};
    use futures::SinkExt;
    use futures::StreamExt;
    use gio::prelude::*;
    use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
    use std::iter::FromIterator;
    use std::slice::Iter;

    #[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
    pub enum TestMessage {
        Foo,
        Bar,
        Baz,
    }

    impl TestMessage {
        pub fn iterator() -> Iter<'static, TestMessage> {
            static MSGS: [TestMessage; 3] = [TestMessage::Foo, TestMessage::Bar, TestMessage::Baz];
            MSGS.iter()
        }
    }

    #[test]
    fn test_gio_socketpair_async() {
        let (local, remote) = socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        )
        .unwrap();

        let local_socket = unsafe { gio::Socket::from_fd(local) }.unwrap();
        let local_connection = local_socket.connection_factory_create_connection();
        let local_stream = local_connection.into_async_read_write().unwrap();
        let (mut local_istream, mut local_ostream) = local_stream.split();

        let remote_socket = unsafe { gio::Socket::from_fd(remote) }.unwrap();
        let remote_connection = remote_socket.connection_factory_create_connection();
        let remote_stream = remote_connection.into_async_read_write().unwrap();
        let (mut remote_istream, mut remote_ostream) = remote_stream.split();

        let ctx = glib::MainContext::default();
        ctx.block_on(async move {
            let data = b"some bytes";
            let write_buffer = BytesMut::from_iter(data.iter());
            local_ostream.write(&write_buffer).await.unwrap();

            let mut read_buffer = BytesMut::with_capacity(write_buffer.len());
            unsafe {
                read_buffer.set_len(read_buffer.capacity());
            }
            remote_istream.read(&mut read_buffer).await.unwrap();

            assert_eq!(write_buffer, read_buffer);
        });
    }

    #[test]
    fn test_gio_socketpair_sync() {
        let (local, remote) = socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        )
        .unwrap();

        let local_socket = unsafe { gio::Socket::from_fd(local) }.unwrap();
        let local_connection = local_socket.connection_factory_create_connection();
        let mut local_istream = local_connection.input_stream();
        let mut local_ostream = local_connection.output_stream();

        let remote_socket = unsafe { gio::Socket::from_fd(remote) }.unwrap();
        let remote_connection = remote_socket.connection_factory_create_connection();
        let mut remote_istream = remote_connection.input_stream();
        let mut remote_ostream = remote_connection.output_stream();

        let data = b"some bytes";
        let write_buffer = BytesMut::from_iter(data.iter());

        local_ostream
            .write_all(&write_buffer, None::<&gio::Cancellable>)
            .unwrap();

        // let mut buffer = vec![0u8; 1024];
        let mut read_buffer = BytesMut::with_capacity(write_buffer.len());
        // unsafe {
        //     read_buffer.set_len(read_buffer.capacity());
        // }

        remote_istream
            .read(&mut read_buffer, None::<&gio::Cancellable>)
            .unwrap();

        assert_eq!(write_buffer, read_buffer);
    }

    #[test]
    fn test_channel() {
        let (local, remote) = socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        )
        .unwrap();

        let local_socket = unsafe { gio::Socket::from_fd(local) }.unwrap();
        let mut local_connection: Connection<TestMessage, _> = connection(local_socket).unwrap();

        let remote_socket = unsafe { gio::Socket::from_fd(remote) }.unwrap();
        let mut remote_connection: Connection<TestMessage, _> = connection(remote_socket).unwrap();

        let ctx = glib::MainContext::default();
        ctx.block_on(async move {
            let data = TestMessage::Bar;

            local_connection.send(data.clone()).await.unwrap();

            let recvd = remote_connection.next().await.unwrap();

            assert_eq!(data, recvd);
        });
    }

    #[test]
    fn test_channel_loop() {
        let (local, remote) = socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        )
        .unwrap();

        let local_socket = unsafe { gio::Socket::from_fd(local) }.unwrap();
        let mut local_connection: Connection<TestMessage, _> = connection(local_socket).unwrap();

        let remote_socket = unsafe { gio::Socket::from_fd(remote) }.unwrap();
        let mut remote_connection: Connection<TestMessage, _> = connection(remote_socket).unwrap();

        let ctx = glib::MainContext::default();
        ctx.block_on(async move {
            for msg in TestMessage::iterator() {
                local_connection.send(msg.clone()).await.unwrap();

                let recvd = remote_connection.next().await.unwrap();

                assert_eq!(msg, &recvd);
            }
        });
    }
}
