use std::cell::RefCell;
use std::os::unix::io::FromRawFd;

use log::*;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
use once_cell::unsync::OnceCell;

use gio::subclass::prelude::ObjectImplExt;
use glib::subclass::prelude::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use glib::Sender;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::WidgetImpl;
use gtk::subclass::widget::WidgetClassSubclassExt;
use webkit2gtk;
use webkit2gtk::traits::WebContextExt;

use crate::core::Action;
use crate::webextension::rpc::RawFdWrap;

use super::theme::WebViewTheme;
use super::web_view::WebViewImpl;
use super::WebView;

#[derive(Debug)]
pub struct MessageWebView {
    pub sender: OnceCell<Sender<Action>>
}

#[glib::object_subclass]
impl ObjectSubclass for MessageWebView {
    const NAME: &'static str = "InoxMessageWebView";
    type Type = super::MessageWebView;
    type ParentType = WebView;

    fn new() -> Self {
        MessageWebView {
            sender: OnceCell::new(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for MessageWebView {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
    }
}
impl WidgetImpl for MessageWebView {}

impl WebViewImpl for MessageWebView {}

impl MessageWebView {

}
