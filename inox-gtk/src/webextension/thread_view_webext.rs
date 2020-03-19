
use std::sync::{Arc, Mutex};
use std::os::unix::net::UnixStream;
use std::os::unix::io::{RawFd, FromRawFd};
use async_std::os::unix::net::{UnixStream as AsyncUnixStream};

use futures::future::{self, Ready};
use futures::stream;

use log::*;
use env_logger;

use glib::Cast;

use glib::variant::Variant;
use gio;
use gio::prelude::*;

use gtk::IconThemeExt;
use webkit2gtk_webextension::{
    DOMDocument,
    DOMDocumentExt,
    DOMElementExt,
    WebExtension,
    WebExtensionExt,
    WebPage,
    WebPageExt,
    DOMNodeExt,
    web_extension_init_with_data
};

use tokio_util::compat::*;

use tarpc;
use tarpc::{server, context};
use tarpc::server::BaseChannel;
use tarpc::rpc::server::Channel;
use tarpc::serde_transport::Transport;

use tokio_serde::formats::{Bincode};

use crate::rpc::handler_respond_with;
use crate::service::{Page, Message};

web_extension_init_with_data!();


/// Init Gtk and logger.
fn init() {
    use std::sync::Once;

    static START: Once = Once::new();

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



pub fn web_extension_initialize(extension: &WebExtension, user_data: Option<&Variant>) {
    init();

    debug!("user data: {:?}", user_data);
    let user_string: Option<RawFd> = user_data.and_then(Variant::get::<RawFd>);

    let socket_addr = user_string.unwrap();

    debug!("socket_addr: {:?}", socket_addr);


    let stream: AsyncUnixStream = unsafe{ UnixStream::from_raw_fd(socket_addr) }.into();
    let transport = Transport::from((stream.compat(), Bincode::default()));

    let webext = ThreadViewWebExt::new(extension.clone());

    let channel = BaseChannel::new(server::Config::default(), transport);

    // channel.respond_with(webex.serve()).execute();
    // let server = handler_respond_with(
    //     server::new(server::Config::default()).incoming(stream::once(future::ready(transport))),
    //     webext.serve());

    let ctx = glib::MainContext::default();
    // ctx.push_thread_default();
    ctx.spawn_local(channel.respond_with(webext.serve()).execute());
    // ctx.pop_thread_default();
}

#[derive(Debug, Clone)]
pub struct ThreadViewWebExt{
    extension: Arc<Mutex<WebExtension>>,
    part_css: Option<String>,
    allowed_uris: Vec<String>,
    indent_messages: bool
}

impl ThreadViewWebExt{

    pub fn new(extension: webkit2gtk_webextension::WebExtension) -> Self{
        let webext = ThreadViewWebExt{
            extension: Arc::new(Mutex::new(extension.clone())),
            part_css: None,
            indent_messages: true,
            allowed_uris: vec![]
        };
        webext
    }


    pub fn on_page_created(&self, _page: &webkit2gtk_webextension::WebPage){

        /* load attachment icon */
        let theme = gtk::IconTheme::get_default().unwrap();
        let _attachment_icon = theme.load_icon(
            "mail-attachment-symbolic",
            ATTACHMENT_ICON_WIDTH,
            gtk::IconLookupFlags::USE_BUILTIN);

        /* load marked icon */
        let _marked_icon = theme.load_icon (
            "object-select-symbolic",
            ATTACHMENT_ICON_WIDTH,
            gtk::IconLookupFlags::USE_BUILTIN);




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



impl Page for ThreadViewWebExt
{
    type AllowRemoteImagesFut = Ready<()>;

    fn allow_remote_images(self, _: context::Context, _name: String)
            -> Self::AllowRemoteImagesFut
    {
        future::ready(())
    }

    type LoadFut = Ready<()>;

    fn load(mut self, _: context::Context,
            html_content: String,
            css_content: String,
            part_css: Option<String>,
            allowed_uris: Vec<String>,
            _use_stdout: bool,
            _use_syslog: bool,
            _disable_log: bool,
            _log_level: String)
            -> Self::LoadFut
    {

        info!("loading page");
        // let page = self.page.as_ref().unwrap();
        // TODO:
        let extension = self.extension.try_lock().unwrap();
        let page = extension.get_page(1).unwrap();
        let document: DOMDocument = page.get_dom_document().unwrap();

        // load html
        let html_element = document.create_element("HTML").unwrap();
        html_element.set_outer_html(&html_content);

        let dom_html_elem = html_element.downcast::<webkit2gtk_webextension::DOMHTMLElement>().unwrap();
        document.set_body(&dom_html_elem);

        // load css style
        info!("loading stylesheet");
        let style_element = document.create_element("STYLE").unwrap();
        let style_text = document.create_text_node(&css_content).unwrap();

        style_element.append_child(&style_text);

        let header = document.get_head().unwrap();
        header.append_child(&style_element);

        info!("loaded page");

        // store part / iframe css for later
        self.part_css = part_css;

        // add allowed uris
        self.allowed_uris = allowed_uris;

        future::ready(())
    }

    type ClearMessagesFut = Ready<()>;
    fn clear_messages(self, _: context::Context) -> Self::ClearMessagesFut {

        debug!("clearing all messages.");

        let extension = self.extension.try_lock().unwrap();
        let page = extension.get_page(1).unwrap();
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

        future::ready(())
    }


    type AddMessageFut = Ready<()>;
    fn add_message(self, _: context::Context, messages: Message) -> Self::AddMessageFut {
        debug!("add_message");

        future::ready(())

    }


    type AddMessagesFut = Ready<()>;
    fn add_messages(self, _: context::Context, messages: Vec<Message>) -> Self::AddMessagesFut {
        debug!("add_messages");

        future::ready(())

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


