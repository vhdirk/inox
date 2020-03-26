use futures::future::{self, FutureExt, Ready};
use futures::io::AsyncReadExt;
use futures::stream;
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::prelude::*;
use std::sync::{Arc, Mutex};

use log::*;
use pretty_env_logger;

use glib::Cast;

use gio;
use gio::prelude::*;
use glib::variant::Variant;

use gtk::IconThemeExt;
use webkit2gtk_webextension::{
    web_extension_init_with_data, DOMDocument, DOMDocumentExt, DOMElementExt, DOMNodeExt,
    WebExtension, WebExtensionExt, WebPage, WebPageExt,
};

use capnp::capability::Promise;
use capnp::primitive_list;
use capnp::Error;

use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::{pry, rpc_twoparty_capnp, RpcSystem};

use crate::rpc::{RawFdWrap, SocketConnectionUtil};
use crate::webext_capnp::page;

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
    let socket_addr: RawFd = user_data.and_then(Variant::get::<RawFd>).unwrap();
    let socket = unsafe { gio::Socket::new_from_fd(RawFdWrap::from_raw_fd(socket_addr)) }.unwrap();

    debug!("socket connected?: {:?}", socket.is_connected());

    let connection = socket.connection_factory_create_connection().unwrap();
    let ostream = connection.get_async_output_stream().unwrap();
    let istream = connection.get_async_input_stream().unwrap();

    let network = Box::new(VatNetwork::new(
        istream,
        ostream,
        rpc_twoparty_capnp::Side::Server,
        Default::default(),
    ));

    let webext = ThreadViewWebExt::new(socket, connection, extension.clone());
    let page_srv = page::ToClient::new(webext).into_client::<capnp_rpc::Server>();
    let rpc_system = RpcSystem::new(network, Some(page_srv.clone().client));

    let ctx = glib::MainContext::default();
    ctx.push_thread_default();
    ctx.spawn_local(rpc_system.then(move |result| {
        // TODO: do something with this result...
        info!("rpc_system done? {:?}", result);
        future::ready(())
    }));
    ctx.pop_thread_default();
}

#[derive(Debug, Clone)]
pub struct ThreadViewWebExt {
    socket: gio::Socket,
    connection: gio::SocketConnection,
    extension: WebExtension,
    part_css: Option<String>,
    allowed_uris: Vec<String>,
    indent_messages: bool,
}

impl ThreadViewWebExt {
    pub fn new(
        socket: gio::Socket,
        connection: gio::SocketConnection,
        extension: webkit2gtk_webextension::WebExtension,
    ) -> Self {
        let webext = ThreadViewWebExt {
            socket,
            connection,
            extension,
            part_css: None,
            indent_messages: true,
            allowed_uris: vec![],
        };
        webext
    }

    pub fn on_page_created(&self, _page: &webkit2gtk_webextension::WebPage) {
        /* load attachment icon */
        let theme = gtk::IconTheme::get_default().unwrap();
        let _attachment_icon = theme.load_icon(
            "mail-attachment-symbolic",
            ATTACHMENT_ICON_WIDTH,
            gtk::IconLookupFlags::USE_BUILTIN,
        );

        /* load marked icon */
        let _marked_icon = theme.load_icon(
            "object-select-symbolic",
            ATTACHMENT_ICON_WIDTH,
            gtk::IconLookupFlags::USE_BUILTIN,
        );

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
}

impl page::Server for ThreadViewWebExt {
    fn allow_remote_images(
        &mut self,
        _params: page::AllowRemoteImagesParams,
        _results: page::AllowRemoteImagesResults,
    ) -> Promise<(), Error> {
        Promise::ok(())
    }

    fn load(
        &mut self,
        params: page::LoadParams,
        _results: page::LoadResults,
    ) -> Promise<(), Error> {
        info!("loading page");
        // let page = self.page.as_ref().unwrap();
        // TODO:
        let page = self.extension.get_page(1).unwrap();
        let document: DOMDocument = page.get_dom_document().unwrap();

        // load html
        let html_element = document.create_element("HTML").unwrap();

        let html_content = pry!(pry!(params.get()).get_html());
        html_element.set_outer_html(html_content);

        let dom_html_elem = html_element
            .downcast::<webkit2gtk_webextension::DOMHTMLElement>()
            .unwrap();
        document.set_body(&dom_html_elem);

        // load css style
        info!("loading stylesheet");
        let style_element = document.create_element("STYLE").unwrap();
        let css_content = pry!(pry!(params.get()).get_css());
        let style_text = document.create_text_node(css_content).unwrap();
        style_element.append_child(&style_text);

        let header = document.get_head().unwrap();
        header.append_child(&style_element);

        info!("loaded page");

        // store part / iframe css for later
        let part_css = pry!(pry!(params.get()).get_part_css());
        self.part_css = Some(part_css.to_string());

        // add allowed uris
        let allowed_uris_r = pry!(pry!(params.get()).get_allowed_uris());
        let allowed_uris = allowed_uris_r
            .iter()
            .filter_map(|x| x.ok())
            .map(ToOwned::to_owned);
        self.allowed_uris = self
            .allowed_uris
            .iter()
            .cloned()
            .chain(allowed_uris)
            .collect();

        Promise::ok(())
    }

    fn clear_messages(
        &mut self,
        params: page::ClearMessagesParams,
        _results: page::ClearMessagesResults,
    ) -> Promise<(), Error> {
        debug!("clearing all messages.");

        let page = self.extension.get_page(1).unwrap();
        let document: DOMDocument = page.get_dom_document().unwrap();

        let container = document.get_element_by_id("message_container").unwrap();
        container.set_inner_html("<span id=\"placeholder\"></span>");

        //   /* reset */
        //   focused_message = "";
        //   focused_element = -1;
        //   messages.clear ();
        //   state = AstroidMessages::State();
        //   allow_remote_resources = false;
        //   indent_messages = false;

        Promise::ok(())
    }
}

fn scroll_by(page: &WebPage, pixels: i64) {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    body.set_scroll_top(body.get_scroll_top() + pixels);
}

fn scroll_bottom(page: &WebPage) {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    body.set_scroll_top(body.get_scroll_height());
}

fn scroll_percentage(page: &WebPage) -> i64 {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    let document = document.get_document_element().unwrap();
    let height = document.get_client_height();
    (body.get_scroll_top() as f64 / (body.get_scroll_height() as f64 - height) * 100.0) as i64
}

fn scroll_top(page: &WebPage) {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    body.set_scroll_top(0);
}
