#![feature(async_closure)]
pub mod webext_capnp {
    include!(concat!(env!("OUT_DIR"), "/resources/webext_capnp.rs"));
}

pub mod rpc;
pub mod thread_view_webext;
// pub mod glib_receiver_future;
