use gio;
use gio::prelude::*;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};

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

pub trait SocketConnectionUtil {
    fn get_async_input_stream(&self)
        -> Option<gio::InputStreamAsyncRead<gio::PollableInputStream>>;
    fn get_async_output_stream(
        &self,
    ) -> Option<gio::OutputStreamAsyncWrite<gio::PollableOutputStream>>;
}

impl SocketConnectionUtil for gio::SocketConnection {
    fn get_async_input_stream(
        &self,
    ) -> Option<gio::InputStreamAsyncRead<gio::PollableInputStream>> {
        self.get_input_stream()
            .and_then(|s| s.dynamic_cast::<gio::PollableInputStream>().ok())
            .and_then(|s| s.into_async_read().ok())
    }

    fn get_async_output_stream(
        &self,
    ) -> Option<gio::OutputStreamAsyncWrite<gio::PollableOutputStream>> {
        self.get_output_stream()
            .and_then(|s| s.dynamic_cast::<gio::PollableOutputStream>().ok())
            .and_then(|s| s.into_async_write().ok())
    }
}
