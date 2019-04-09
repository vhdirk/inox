use std::thread;
use log::*;
use env_logger;
use ipc_channel::ipc;
use bincode;
use serde_derive::{Serialize, Deserialize};
use glib::Cast;
use glib::Object;
use glib::closure::Closure;
use glib::variant::Variant;
use gio;
use gio::{SocketClientExt, IOStreamExt};
use webkit2gtk_webextension::{
    DOMDocumentExt,
    DOMElementExt,
    DOMEventTargetExt,
    DOMMouseEvent,
    DOMMouseEventExt,
    WebExtension,
    WebExtensionExt,
    WebPage,
    WebPageExt,
    web_extension_init_with_data
};
use relm::init as relm_init;
use relm::Component;
use crate::protocol::MessageInputStream;

#[derive(Serialize, Deserialize)]
pub enum IpcMsg{

}

web_extension_init_with_data!();


pub struct ThreadViewWebExt{
    extension: WebExtension
}


/// Init Gtk and logger.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        env_logger::init();
    });
}


pub fn web_extension_initialize(extension: &WebExtension, user_data: Option<&Variant>) {
    init();

    let user_string: Option<String> = user_data.and_then(Variant::get_str).map(ToOwned::to_owned);
    debug!("user string: {:?}", user_string);

    let socket_addr = user_string.unwrap();

    let gsock_addr = gio::UnixSocketAddress::new_with_type(
        gio::UnixSocketAddressPath::Abstract(socket_addr.as_ref()));

    // connect to socket
    let cli = gio::SocketClient::new();
    let sock = cli.connect(&gsock_addr, None::<&gio::Cancellable>).unwrap();

    let istream = sock.get_input_stream().unwrap();
    let ostream = sock.get_output_stream().unwrap();

    info!("stream:{:?}", istream);

    istream.read_message(None::<&gio::Cancellable>);



    extension.connect_page_created(|_, page| {


        
        page.connect_document_loaded(|page| {
            println!("Page {} created for {:?}", page.get_id(), page.get_uri());
            let document = page.get_dom_document().unwrap();
            println!("URL: {:?}", document.get_url());
            println!("Title: {:?}", document.get_title());
            document.set_title("My Web Page");

            let handler = Closure::new(|values| {
                if let Some(event) = values[1].get::<Object>() {
                    // if let Ok(mouse_event) = event.downcast::<DOMMouseEvent>() {
                    //     println!("Click at ({}, {})", mouse_event.get_x(), mouse_event.get_y());
                    // }
                }
                None
            });
            document.add_event_listener_with_closure("click", &handler, false);

            println!("{}%", scroll_percentage(page));
            scroll_by(page, 45);

            println!("{}%", scroll_percentage(page));
            scroll_bottom(page);

            println!("{}%", scroll_percentage(page));
            scroll_top(page);

            println!("{}%", scroll_percentage(page));
        });
    });
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


