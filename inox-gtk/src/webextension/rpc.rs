use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use log::*;
use gio;
use gio::prelude::*;

pub struct RawFdWrap(RawFd);

impl IntoRawFd for RawFdWrap {
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

impl FromRawFd for RawFdWrap {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self(fd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        use pretty_env_logger;
        let _ = env_logger::builder().is_test(true).try_init();
    }
    #[test]
    fn test_gio_socketpair() {
        init();

        use futures::io::{AsyncReadExt, AsyncWriteExt};
        use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockProtocol, SockType};

        let (local, remote) = socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        )
        .unwrap();

        let local_socket = unsafe { gio::Socket::from_fd(RawFdWrap::from_raw_fd(local)) }.unwrap();
        let local_connection = local_socket.connection_factory_create_connection();
        let local_stream = local_connection.into_async_read_write().unwrap();
        let (mut local_istream, mut local_ostream) = local_stream.split();

        let remote_socket = unsafe { gio::Socket::from_fd(RawFdWrap::from_raw_fd(remote)) }.unwrap();
        let remote_connection = remote_socket.connection_factory_create_connection();
        let remote_stream = remote_connection.into_async_read_write().unwrap();
        let (mut remote_istream, mut remote_ostream) = remote_stream.split();

        let ctx = glib::MainContext::default();
        ctx.block_on(async move {
            let data = b"some bytes";
            local_ostream.write(data).await.unwrap();

            let mut buffer = [0u8; 10];
            remote_istream.read(&mut buffer).await.unwrap();

            assert_eq!(data, &buffer);
        });

    }
}