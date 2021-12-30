use async_std::os::unix::net::UnixStream;
use glib::subclass::prelude::ObjectSubclassExt;
use gmime::traits::MessageExt;
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

use super::message_web_view_imp as imp;
use super::theme::WebViewTheme;
use super::web_view_imp;
use super::WebView;

// Wrap imp::ThreadList into a usable gtk-rs object
glib::wrapper! {
    pub struct MessageWebView(ObjectSubclass<imp::MessageWebView>)
        @extends WebView, gtk::Widget;
}

impl MessageWebView {
    pub fn new(sender: Sender<Action>) -> Self {
        let obj: Self = glib::Object::new(&[]).expect("Failed to create MessageWebView");
        obj
    }

    pub fn load_html(&self, body: &str) {
        info!("render: loading html..");
        // TODO: make proper call to parent
        let web_view = self.clone().upcast::<WebView>();
        let web_view_imp = web_view_imp::WebView::from_instance(&web_view);
        web_view_imp.web_view.load_html(body, None)
    }
}
