use futures::{FutureExt, SinkExt, StreamExt, TryFutureExt, TryStreamExt};
use futures_codec::{FramedRead, FramedWrite, LinesCodec};
use gio;
use gio::prelude::*;
use glib;

use jsonrpc_core_client::transports::duplex::duplex;
use jsonrpc_core_client::{RpcChannel, RpcError};

pub fn connect(
    connection: gio::SocketConnection,
) -> Result<RpcChannel, RpcError> {
    let rw = connection
        .into_async_read_write()
        .expect("Could not convert connection into readwrite");

    let istream = rw.input_stream().clone();
    let read = istream
        .into_async_read()
        .expect("Could not create asyncread stream");

    let ostream = rw.output_stream().clone();
    let write = ostream
        .into_async_write()
        .expect("Could not create asyncwrite stream");

    let stream = FramedRead::new(read, LinesCodec);
    let sink = FramedWrite::new(write, LinesCodec);

    let sink = sink.sink_map_err(|e| RpcError::Other(Box::new(e)));
    let stream = stream.map_err(|e| log::error!("IPC stream error: {}", e));

    let (client, sender) = duplex(
        Box::pin(sink),
        Box::pin(
            stream
                .take_while(|x| futures::future::ready(x.is_ok()))
                .map(|x| x.expect("Stream is closed upon first error.")),
        ),
    );

    let ctx = glib::MainContext::default();
    ctx.with_thread_default(move || {
        let ctx = glib::MainContext::default();
        ctx.spawn_local(client.map(|a| ()));
    });

    Ok(sender)
}
