
use crate::widgets::message_web_view::page_client::PageClient;
use crate::widgets::message_web_view::theme::MessageWebViewTheme;
use crate::core::Action;
use crate::webextension::rpc::RawFdWrap;
use gio::subclass::prelude::ObjectImplExt;
use glib::subclass::prelude::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use glib::Sender;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::WidgetImpl;
use gtk::subclass::widget::WidgetClassSubclassExt;
use log::*;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;
use std::os::unix::io::FromRawFd;
use webkit2gtk;
use webkit2gtk::traits::WebContextExt;

fn initialize_web_extensions(ctx: &webkit2gtk::WebContext) -> gio::Socket {
    info!("initialize_web_extensions");
    let cur_exe = std::env::current_exe().unwrap();
    let exe_dir = cur_exe.parent().unwrap();
    let extdir = exe_dir.to_string_lossy();

    info!("setting web extensions directory: {:?}", extdir);
    ctx.set_web_extensions_directory(&extdir);

    let (local, remote) = socketpair(
        AddressFamily::Unix,
        SockType::Stream,
        None,
        SockFlag::empty(),
    )
    .unwrap();

    ctx.set_web_extensions_initialization_user_data(&remote.to_variant());

    unsafe { gio::Socket::from_fd(RawFdWrap::from_raw_fd(local)) }.unwrap()
}

#[derive(Clone, Debug)]
pub struct MessageWebView {
    pub sender: OnceCell<Sender<Action>>,
    pub webview: webkit2gtk::WebView,
    pub webcontext: webkit2gtk::WebContext,
    pub page_client: PageClient,
    pub theme: MessageWebViewTheme,
}

// impl Default for MessageWebView {
//     fn default() -> Self {
//         Self {
//             sender: OnceCell::new(),
//             webview: webkit2gtk::WebView::new(),
//             webcontext: webkit2gtk::WebContext::default().unwrap(),
//             page_client: OnceCell::new(),
//             theme: MessageWebViewTheme::default(),
//             thread: RefCell::new(None),
//         }
//     }
// }

#[glib::object_subclass]
impl ObjectSubclass for MessageWebView {
    const NAME: &'static str = "InoxMessageWebView";
    type Type = super::MessageWebView;
    type ParentType = gtk::Widget;

    fn new() -> Self {
        let webcontext = webkit2gtk::WebContext::default().unwrap();
        let webview = webkit2gtk::WebViewBuilder::new()
            .web_context(&webcontext)
            .user_content_manager(&webkit2gtk::UserContentManager::new())
            .build();
        let stream = initialize_web_extensions(&webcontext);
        let page_client = PageClient::new(&stream);

        MessageWebView {
            sender: OnceCell::new(),
            webview,
            webcontext,
            page_client,
            theme: MessageWebViewTheme::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for MessageWebView {
    fn constructed(&self, obj: &Self::Type) {
        self.webview.set_parent(obj);
        // Setup
        // obj.setup_model();
        // obj.setup_callbacks();
        // obj.setup_columns();

        // imp.column_view.set_parent(&imp.window);
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.webview.unparent();
    }
}
impl WidgetImpl for MessageWebView {}

impl MessageWebView {

}
