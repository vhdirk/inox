use once_cell::sync::OnceCell;
use gmime::traits::MessageExt;
use std::fmt;
use async_std::os::unix::net::UnixStream;
use futures::future::{self, FutureExt, Ready, TryFutureExt};
use futures::io::AsyncReadExt;
use futures::Future;
use std::io;
use std::rc::Rc;

use glib::subclass::prelude::*;
use gio::prelude::*;
use gio::traits::{IOStreamExt, SocketExt};

use log::*;
use notmuch;

use crate::webextension::channel;
use crate::webextension::protocol::WebViewMessage;
use super::theme::WebViewTheme;
use super::web_view_client_imp as imp;

// Wrap imp::ThreadList into a usable gtk-rs object
glib::wrapper! {
    pub struct WebViewClient(ObjectSubclass<imp::WebViewClient>);
}



impl WebViewClient {
    pub fn new(socket: &gio::Socket) -> Self {
        let client = glib::Object::new(&[]).expect("Failed to create MessageWebView");
        let imp = imp::WebViewClient::from_instance(&client);


        let connection = channel::connection::<WebViewMessage>(socket.clone()).unwrap();

        imp.connection.set(Rc::new(connection));
        // let (istream, ostream) = stream.split();
        // let network = Box::new(VatNetwork::new(
        //     istream,
        //     ostream,
        //     rpc_twoparty_capnp::Side::Client,
        //     Default::default(),
        // ));

        // let mut rpc_system = RpcSystem::new(network, None);
        // let client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        // let ctx = glib::MainContext::default();
        // ctx.with_thread_default(|| {
        //     ctx.spawn_local(rpc_system.then(move |result| {
        //         debug!("rpc system result {:?}", result);
        //         // TODO: do something with this result...
        //         future::ready(())
        //     }))
        // });

        client
    }
}
