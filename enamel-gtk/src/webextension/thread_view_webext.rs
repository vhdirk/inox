use std::{mem, thread};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use log::*;
use env_logger;
use serde_derive::{Serialize, Deserialize};
use glib::Cast;
use glib::Object;
use glib::closure::Closure;
use glib::variant::Variant;
use gio;
use gio::prelude::*;
use gio::{SocketClientExt, IOStreamExt, InputStreamExtManual, OutputStreamExtManual};
use gtk::IconThemeExt;
use webkit2gtk_webextension::{
    DOMDocument,
    DOMDocumentExt,
    DOMElementExt,
    DOMEventTargetExt,
    DOMMouseEvent,
    DOMMouseEventExt,
    WebExtension,
    WebExtensionExt,
    WebPage,
    WebPageExt,
    DOMNodeExt,
    web_extension_init_with_data
};

use std::os::unix::net::UnixStream;
use async_std::os::unix::net::{UnixStream as AsyncUnixStream};
use async_std::os::unix::io::{AsRawFd, FromRawFd};
use futures::future::{self, Future, FutureExt};

use capnp::{Error, primitive_list};
use capnp::capability::Promise;

use capnp_rpc::{RpcSystem, rpc_twoparty_capnp, pry};
use capnp_rpc::twoparty::VatNetwork;

use crate::webext_capnp::page;

web_extension_init_with_data!();


/// Init Gtk and logger.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        env_logger::init();

        // we're being called in an environment that has gtk already
        // initialized, but gtk-rs does not know that.
        // TODO: move this into webkit2gtk-webextension
        unsafe {
            gtk::set_initialized();
        }
    });
}

const ATTACHMENT_ICON_WIDTH: i32 = 35;


#[derive(Debug, Clone)]
pub struct ThreadViewWebExt{
    extension: WebExtension,
    page_fut: RefCell<Option<webkit2gtk_webextension::WebPage>>,

    allowed_uris: Vec<String>
}

impl ThreadViewWebExt{

    pub fn new(extension: webkit2gtk_webextension::WebExtension) -> Self{
        let webext = ThreadViewWebExt{
            extension: extension.clone(),
            page_fut: RefCell::new(None),

            allowed_uris: vec![]
        };
        webext
    }


    pub fn on_page_created(&self, page: &webkit2gtk_webextension::WebPage){
        
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


pub fn web_extension_initialize(extension: &WebExtension, user_data: Option<&Variant>) {
    init();


    /* load attachment icon */
    let theme = gtk::IconTheme::get_default().unwrap();
    let attachment_icon = theme.load_icon(
        "mail-attachment-symbolic",
        ATTACHMENT_ICON_WIDTH,
        gtk::IconLookupFlags::USE_BUILTIN);

    /* load marked icon */
    let marked_icon = theme.load_icon (
        "object-select-symbolic",
        ATTACHMENT_ICON_WIDTH,
        gtk::IconLookupFlags::USE_BUILTIN);


    let user_string: Option<String> = user_data.and_then(Variant::get_str).map(ToOwned::to_owned);
    debug!("user string: {:?}", user_string);

    let socket_addr = user_string.unwrap();

    let mut rstream_sync = UnixStream::connect(socket_addr).unwrap();
    let mut wstream_sync = rstream_sync.try_clone().unwrap();

    let rstream: AsyncUnixStream = rstream_sync.into();
    let wstream: AsyncUnixStream = wstream_sync.into();

    let webext = ThreadViewWebExt::new(extension.clone());

    let page_srv = page::ToClient::new(webext).into_client::<::capnp_rpc::Server>();

    let network = VatNetwork::new(rstream,
                                  wstream,
                                  rpc_twoparty_capnp::Side::Server,
                                  Default::default());

    let rpc_system = RpcSystem::new(Box::new(network), Some(page_srv.clone().client));
    // let client: page::Server = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Client);


    let ctx = glib::MainContext::default();

    ctx.push_thread_default();
    ctx.spawn_local(rpc_system.then(move |result| {
        // TODO: do something with this result...
        info!("rpc_system done? {:?}", result);
        future::ready(())
    }));
    ctx.pop_thread_default();
}

impl page::Server for ThreadViewWebExt
{
    fn allow_remote_images(&mut self,
            params: page::AllowRemoteImagesParams,
            mut results: page::AllowRemoteImagesResults)
            -> Promise<(), Error>
    {
        Promise::ok(())
    }

    fn load(&mut self,
            params: page::LoadParams,
            mut results: page::LoadResults)
            -> Promise<(), Error>
    {

        info!("loading page");
        // let page = self.page.as_ref().unwrap();
        // TODO: 
        let page = self.extension.get_page(1).unwrap();
        let document: DOMDocument = page.get_dom_document().unwrap();

        // load html
        let html_element = document.create_element("HTML").unwrap();

        let html_content = pry!(pry!(params.get()).get_html());
        html_element.set_outer_html(html_content);

        let dom_html_elem = html_element.downcast::<webkit2gtk_webextension::DOMHTMLElement>().unwrap();
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

        //   /* store part / iframe css for later */
        //   part_css = s.part_css ();

        // add allowed uris
        let allowed_uris_r = pry!(pry!(params.get()).get_allowed_uris());
        let allowed_uris = allowed_uris_r.iter().filter_map(|x| x.ok()).map(ToOwned::to_owned);
        self.allowed_uris = self.allowed_uris.iter().cloned().chain(allowed_uris).collect();

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


