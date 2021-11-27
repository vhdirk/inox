use crate::core::Message;
use gmime::traits::MessageExt;
use async_std::os::unix::net::UnixStream;
use glib::subclass::prelude::ObjectSubclassExt;
use std::cell::RefCell;
use std::os::unix::io::FromRawFd;
use std::os::unix::prelude::*;

use log::*;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockProtocol, SockType};

use gio;
use gio::prelude::*;

use glib;
use glib::Sender;
use gmime;
use gmime::traits::{ParserExt, PartExt};
use gtk;
use gtk::prelude::*;
use webkit2gtk;
use webkit2gtk::traits::{
    NavigationPolicyDecisionExt, PolicyDecisionExt, SettingsExt, URIRequestExt, WebContextExt,
    WebViewExt,
};

use crate::core::Action;
use crate::spawn;
use crate::webextension::rpc::RawFdWrap;
use crate::core::Thread;

use super::theme::WebViewTheme;
use super::WebView;
use super::message_web_view_imp as imp;
use super::web_view_imp;

// Wrap imp::ThreadList into a usable gtk-rs object
glib::wrapper! {
    pub struct MessageWebView(ObjectSubclass<imp::MessageWebView>)
        @extends WebView, gtk::Widget;
}

// MessageWebView implementation itself
impl MessageWebView {
    pub fn new(sender: Sender<Action>) -> Self {
        glib::Object::new(&[]).expect("Failed to create MessageWebView")
    }

    pub fn setup_signals(&self) {
        // let web_view = self.clone().upcast::<WebView>().unwrap();
        // let web_view_imp = web_view_imp::WebView::from_instance(&web_view);

        // let imp = imp::MessageWebView::from_instance(self);
        // let self_ = self.clone();
        // web_view_imp.web_view.connect_load_changed(move |_, event| {
        //     let mut mself = self_.clone();

        //     mself.load_changed(event);
        // });

    }

    // fn load_changed(&mut self, event: webkit2gtk::LoadEvent) {
    //     info!("MessageWebView: load changed: {:?}", event);
    //     let imp = imp::MessageWebView::from_instance(self);

    //     match event {
    //         webkit2gtk::LoadEvent::Finished => {
    //             if imp.page_client.is_ready() {
    //                 self.ready_to_render();
    //             }
    //         }
    //         _ => (),
    //     }
    // }


    pub fn load_html(&self, body: &str) {
        info!("render: loading html..");
        // TODO: make proper call to parent
        let web_view = self.clone().upcast::<WebView>();
        let web_view_imp = web_view_imp::WebView::from_instance(&web_view);
        web_view_imp.web_view.load_html(body, None)
    }


}