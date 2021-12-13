use async_std::os::unix::net::UnixStream;
use async_std::task;
use futures::channel::{mpsc, oneshot};
use futures::future::BoxFuture;
use futures::future::{self, FutureExt, Ready, TryFuture, TryFutureExt};
use futures::io::AsyncReadExt;
use futures::stream;
use futures::AsyncWriteExt;
use futures::Future;
use futures::SinkExt;
use gio::prelude::IOStreamExtManual;
use gio::prelude::SocketExt;
use gio::Socket;
use log::*;
use serde::Serialize;
use std::cell::RefCell;
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::prelude::*;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use glib::Cast;

use glib::variant::Variant;

use webkit2gtk_webextension::traits::{
    DOMDocumentExt, DOMElementExt, DOMNodeExt, WebExtensionExt, WebPageExt,
};
use webkit2gtk_webextension::{web_extension_init_with_data, DOMDocument, WebExtension, WebPage};

// use crate::glib_receiver_future::GLibReceiverFuture;
use super::connection::{self, connection, Connection};
use super::protocol::WebViewMessage;
use super::rpc::RawFdWrap;

web_extension_init_with_data!();

/// Init Gtk and logger.
fn init() {
    use std::sync::Once;

    static START: Once = Once::new();

    START.call_once(|| {
        pretty_env_logger::init();

        // we're being called in an environment that has gtk already
        // initialized, but gtk-rs does not know that.
        // TODO: move this into webkit2gtk-webextension
        unsafe {
            gtk::set_initialized();
        }
    });
}

const ATTACHMENT_ICON_WIDTH: i32 = 35;

pub fn web_extension_initialize(extension: &WebExtension, user_data: Option<&Variant>) {
    init();

    debug!("user data: {:?}", user_data);
    // let socket_address: String = user_data.and_then(Variant::get::<String>).unwrap();

    let socket_addr: RawFd = user_data.and_then(Variant::get::<RawFd>).unwrap();
    let socket = unsafe { gio::Socket::from_fd(socket_addr) }.unwrap();

    debug!("socket connected?: {:?}", socket.is_connected());

    let webext = WebViewExtension::new(socket, extension.clone());
    webext.init();

    // let extension = extension.clone();

    // let ctx = glib::MainContext::new();

    // let ctx = glib::MainContext::default();
    // ctx.with_thread_default(move || {
    //     let ctx = glib::MainContext::default();

    //     ctx.spawn_local(async move {
    //         let stream = UnixStream::connect(socket_address).await.unwrap();

    //         let connection = Connection::new(stream);
    //         let webext = WebViewExtension::new(connection, extension.clone());
    //         webext.init();
    //     });
    // });
}

#[derive(Clone, Debug)]
pub struct WebViewExtension {
    connection:
        Rc<RefCell<Connection<WebViewMessage, gio::IOStreamAsyncReadWrite<gio::SocketConnection>>>>,
    extension: WebExtension,
}

// fn page_loaded_future(
//     extension: webkit2gtk_webextension::WebExtension,
// ) -> Pin<Box<dyn Future<Output = webkit2gtk_webextension::WebPage> + 'static>> {
//     let (sender, receiver) = mpsc::unbounded::<webkit2gtk_webextension::WebPage>();

//     extension.connect_page_created(move |_, page| {
//         info!("page created: {:?}", page.id());
//         sender.unbounded_send(page.clone());
//     });

//     Box::pin(receiver.into_future())
// }

impl WebViewExtension {
    pub fn new(socket: gio::Socket, extension: webkit2gtk_webextension::WebExtension) -> Self {
        debug!("create webext {:?}", socket);

        let connection = connection::<WebViewMessage>(socket).unwrap();
        debug!("created connection {:?}", connection);

        WebViewExtension {
            connection: Rc::new(RefCell::new(connection)),
            extension,
        }
    }

    pub fn init(&self) {
        let this = self.clone();
        self.extension.connect_page_created(move |_, page| {
            debug!("on page created {:?}", this);

            this.on_page_created(page)
        });
    }

    pub async fn serve(&self) {}

    pub fn preferred_height(&self, page: &webkit2gtk_webextension::WebPage) -> Option<i64> {
        // Return the scroll height of the HTML element since the BODY
        // may have margin/border/padding and we want to know
        // precisely how high the widget needs to be to avoid
        // scrolling.
        page.dom_document()
            .map(|dom_document| dom_document.document_element().unwrap().scroll_height())
    }

    pub fn on_document_loaded(&self, page: &webkit2gtk_webextension::WebPage) {
        if let Some(height) = self.preferred_height(page) {

            let this = self.clone();
            let ctx = glib::MainContext::default();
            ctx.with_thread_default(move || {
                let ctx = glib::MainContext::default();
                ctx.spawn_local(async move {
                    this.connection
                        .borrow_mut()
                        .send(WebViewMessage::PreferredHeightChanged(height))
                        .await;

                    this.connection
                        .borrow_mut()
                        .send(WebViewMessage::ContentLoaded)
                        .await;
                    ()
                });
            });
        }
    }

    pub fn on_page_created(&self, page: &webkit2gtk_webextension::WebPage) {
        let this = self.clone();
        page.connect_document_loaded(move |page| {
            debug!("Page {:?} created for {:?}", page.id(), page.uri());
            this.on_document_loaded(page);
        });

        // page.console_message_sent.connect(on_console_message);
        // page.send_request.connect(on_send_request);
        // page.user_message_received.connect(on_page_message_received);

        // self.page = Some(page.clone());
        /* load attachment icon */
        let theme = gtk::IconTheme::default();
        // let _attachment_icon = theme.load_icon(
        //     "mail-attachment-symbolic",
        //     ATTACHMENT_ICON_WIDTH,
        //     gtk::IconLookupFlags::USE_BUILTIN,
        // );

        // /* load marked icon */
        // let _marked_icon = theme.load_icon(
        //     "object-select-symbolic",
        //     ATTACHMENT_ICON_WIDTH,
        //     gtk::IconLookupFlags::USE_BUILTIN,
        // );

        // race condition with rpc system here
        // info!("WEEEEEE {:?}{:?}{:?}", self, page, page.get_id());
        // *(self.page.borrow_mut()) = Some(page.clone());

        // page.connect_document_loaded(|page| {
        //     println!("Page {} created for {:?}", page.get_id(), page.get_uri());
        //     let document = page.get_dom_document().unwrap();
        //     println!("URL: {:?}", document.get_url());
        //     println!("Title: {:?}", document.get_title());
        //     document.set_title("My Web Page");

        //     let handler = Closure::new(|values| {
        //         if let Ok(Some(event)) = values[1].get::<Object>() {
        //             // if let Ok(mouse_event) = event.downcast::<DOMMouseEvent>() {
        //             //     println!("Click at ({}, {})", mouse_event.get_x(), mouse_event.get_y());
        //             // }
        //         }
        //         None
        //     });
        //     document.add_event_listener_with_closure("click", &handler, false);

        //     println!("{}%", scroll_percentage(page));
        //     scroll_by(page, 45);

        //     println!("{}%", scroll_percentage(page));
        //     scroll_bottom(page);

        //     println!("{}%", scroll_percentage(page));
        //     scroll_top(page);

        //     println!("{}%", scroll_percentage(page));
        // });
    }

    // async fn _load(
    //     self,
    //     params: page::LoadParams,
    //     _results: page::LoadResults,
    // ) -> Result<(), Error> {
    //     // page_loaded_future(self.extension.clone()).await;
    //     // info!("page id: {}:?", page_id);

    //     Ok(())
    // }

    // async fn _clear_messages(
    //     self,
    //     params: page::ClearMessagesParams,
    //     _results: page::ClearMessagesResults,
    // ) -> Result<(), Error> {
    //     // page_loaded_future(self.extension.clone()).await;

    //     // info!("page id: {}:?", page_id);

    //     debug!("clearing all messages");

    //     Ok(())
    // }
}

// impl page::Server for WebViewExtension {
//     fn allow_remote_images(
//         &mut self,
//         _params: page::AllowRemoteImagesParams,
//         _results: page::AllowRemoteImagesResults,
//     ) -> Promise<(), Error> {
//         Promise::ok(())
//     }

//     fn load(&mut self, params: page::LoadParams, results: page::LoadResults) -> Promise<(), Error> {
//         // info!("loading page. page: {:?}", self.page);

//         Promise::from_future(self.clone()._load(params, results))

//         // let page = self.extension.get_page(0).unwrap();
//         // let document: DOMDocument = page.get_dom_document().unwrap();

//         // // load html
//         // let html_element = document.create_element("HTML").unwrap();

//         // let html_content = pry!(pry!(params.get()).get_html());
//         // html_element.set_outer_html(html_content);

//         // let dom_html_elem = html_element
//         //     .downcast::<webkit2gtk_webextension::DOMHTMLElement>()
//         //     .unwrap();
//         // document.set_body(&dom_html_elem);

//         // // load css style
//         // info!("loading stylesheet");
//         // let style_element = document.create_element("STYLE").unwrap();
//         // let css_content = pry!(pry!(params.get()).get_css());
//         // let style_text = document.create_text_node(css_content).unwrap();
//         // style_element.append_child(&style_text);

//         // let header = document.get_head().unwrap();
//         // header.append_child(&style_element);

//         // info!("loaded page");

//         // // store part / iframe css for later
//         // let part_css = pry!(pry!(params.get()).get_part_css());
//         // self.part_css = Some(part_css.to_string());

//         // // add allowed uris
//         // let allowed_uris_r = pry!(pry!(params.get()).get_allowed_uris());
//         // let allowed_uris = allowed_uris_r
//         //     .iter()
//         //     .filter_map(|x| x.ok())
//         //     .map(ToOwned::to_owned);
//         // self.allowed_uris = self
//         //     .allowed_uris
//         //     .iter()
//         //     .cloned()
//         //     .chain(allowed_uris)
//         //     .collect();

//         // let mut _self = self.clone();
//         // Promise::from_future(_self._load(params, results))

//         // Promise::ok(())
//     }

//     fn clear_messages(
//         &mut self,
//         params: page::ClearMessagesParams,
//         results: page::ClearMessagesResults,
//     ) -> Promise<(), Error> {
//         Promise::from_future(self.clone()._clear_messages(params, results))

//         // debug!("clearing all messages. page: {:?}", self.page);

//         // let page = self.extension.get_page(0).unwrap();
//         // let document: DOMDocument = page.get_dom_document().unwrap();

//         // let container = document.get_element_by_id("message_container").unwrap();
//         // container.set_inner_html("<span id=\"placeholder\"></span>");

//         //   /* reset */
//         //   focused_message = "";
//         //   focused_element = -1;
//         //   messages.clear ();
//         //   state = AstroidMessages::State();
//         //   allow_remote_resources = false;
//         //   indent_messages = false;
//     }
// }

fn scroll_by(page: &WebPage, pixels: i64) {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    body.set_scroll_top(body.scroll_top() + pixels);
}

fn scroll_bottom(page: &WebPage) {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    body.set_scroll_top(body.scroll_height());
}

fn scroll_percentage(page: &WebPage) -> i64 {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    let document = document.document_element().unwrap();
    let height = document.client_height();
    (body.scroll_top() as f64 / (body.scroll_height() as f64 - height) * 100.0) as i64
}

fn scroll_top(page: &WebPage) {
    let document = page.dom_document().unwrap();
    let body = document.body().unwrap();
    body.set_scroll_top(0);
}
