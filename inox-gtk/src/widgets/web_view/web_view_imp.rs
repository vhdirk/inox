use async_std::io::Error;
use async_std::os::unix::net::{UnixListener, UnixStream};
use futures::future::{self, Ready};
use futures::{
    AsyncReadExt, AsyncWriteExt, FutureExt, Sink, Stream, StreamExt, TryFuture, TryFutureExt,
    TryStream, TryStreamExt,
};
use nix::unistd::dup;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use log::*;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
use once_cell::unsync::OnceCell;
use std::process;

use gio::subclass::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::subclass::widget::WidgetClassSubclassExt;
use ipc_channel::asynch::IpcStream;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcReceiver, IpcSender};
use std::os::unix::io::FromRawFd;
use std::path::Path;
use webkit2gtk;
use webkit2gtk::traits::{
    NavigationPolicyDecisionExt, PolicyDecisionExt, SettingsExt, URIRequestExt, WebContextExt,
    WebViewExt as WebKitWebViewExt,
};

use crate::core::Action;
use crate::webextension::connection::{self, connection, Connection};
use crate::webextension::protocol::WebViewMessage;
use crate::webextension::rpc::RawFdWrap;

use super::theme::WebViewTheme;

pub type WebViewInstance = super::WebView;

/** URI Scheme and delimiter for internal resource loads. */
const INTERNAL_URL_PREFIX: &str = "inox:";

/** URI for internal message body page loads. */
// TODO: create from INTERNAL_URL_PREFIX
const INTERNAL_URL_BODY: &str = "inox:body";

const SOCKET_PATH: &str = "/tmp/inox-gtk-webview-socket";

fn initialize_web_extension(
    ctx: &webkit2gtk::WebContext,
) -> Result<Connection<WebViewMessage, gio::IOStreamAsyncReadWrite<gio::SocketConnection>>, Error> {
    // ) -> Result<String, Error> {
    info!("initialize_web_extension");
    let cur_exe = std::env::current_exe().unwrap();
    let exe_dir = cur_exe.parent().unwrap();
    let extdir = exe_dir.to_string_lossy();
    info!("cur_exe: {:?}", extdir);

    info!("setting web extensions directory: {:?}", extdir);
    ctx.set_web_extensions_directory(&extdir);

    let (local, remote) = socketpair(
        AddressFamily::Unix,
        SockType::Stream,
        None,
        SockFlag::empty(),
    )
    .unwrap();

    let socket = unsafe { gio::Socket::from_fd(RawFdWrap::from_raw_fd(local)) }.unwrap();

    let connection = connection::<WebViewMessage>(socket.clone());

    ctx.set_web_extensions_initialization_user_data(&remote.to_variant());

    connection

    // // this only allows one webview per process, but makes cleanup easier. Good enough for now.
    // let socket_path = format!("{}-{}", SOCKET_PATH, process::id());
    // let socket = Path::new(&socket_path);

    // // Delete old socket if necessary
    // if socket.exists() {
    //     fs::remove_file(&socket).unwrap();
    // }

    // ctx.set_web_extensions_initialization_user_data(&socket_path.to_variant());

    // // bind but don't do anything with the socket
    // // UnixListener::bind(&socket)

    // Ok(socket_path)
}

#[repr(C)]
pub struct WebViewClass {
    pub parent_class: webkit2gtk::ffi::WebKitWebViewClass,
    pub load_html: fn(&WebViewInstance, &str),
}

unsafe impl ClassStruct for WebViewClass {
    type Type = WebView;
}

fn load_html_default_trampoline(this: &WebViewInstance, html: &str) {
    WebView::from_instance(this).load_html(this, html)
}

pub fn web_view_load_html(this: &WebViewInstance, html: &str) {
    let klass = this.class();
    (klass.as_ref().load_html)(this, html)
}

pub struct WebView {
    pub web_view: webkit2gtk::WebView,
    pub web_context: webkit2gtk::WebContext,
    pub settings: webkit2gtk::Settings,
    pub connection:
        RefCell<Connection<WebViewMessage, gio::IOStreamAsyncReadWrite<gio::SocketConnection>>>,
    // pub socket_path: String,
    // pub connection: RefCell<Option<Connection<WebViewMessage, UnixStream>>>,
    pub theme: WebViewTheme,
}

#[glib::object_subclass]
impl ObjectSubclass for WebView {
    const NAME: &'static str = "InoxWebView";
    type Type = super::WebView;
    type ParentType = gtk::Widget;
    type Class = WebViewClass;

    fn new() -> Self {
        let web_context = webkit2gtk::WebContext::default().unwrap();
        let web_view = webkit2gtk::WebViewBuilder::new()
            .web_context(&web_context)
            .user_content_manager(&webkit2gtk::UserContentManager::new())
            .build();
        // let socket_path = initialize_web_extension(&web_context).unwrap();
        let connection = initialize_web_extension(&web_context);

        let settings = WebKitWebViewExt::settings(&web_view).unwrap();

        WebView {
            web_view,
            web_context,
            settings,
            connection: RefCell::new(connection.unwrap()),
            // socket_path,
            // connection: RefCell::new(None),
            theme: WebViewTheme::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();

        klass.load_html = load_html_default_trampoline;
    }
}

impl ObjectImpl for WebView {
    fn constructed(&self, obj: &Self::Type) {
        // Start socket stuff first?
        self.init_extension_message_receiver();

        self.web_view.set_parent(obj);
        self.web_view.set_hexpand(true);
        self.web_view.set_vexpand(true);
        self.web_view.show();

        obj.set_hexpand(true);
        obj.set_vexpand(true);
        obj.show();

        self.web_context
            .set_cache_model(webkit2gtk::CacheModel::DocumentViewer);

        // self.settings.set_enable_scripts(true);
        // self.settings.set_enable_java_applet(false);
        self.settings.set_enable_plugins(false);
        self.settings.set_auto_load_images(true);
        self.settings.set_enable_dns_prefetching(false);
        self.settings.set_enable_fullscreen(false);
        self.settings.set_enable_html5_database(false);
        self.settings.set_enable_html5_local_storage(false);
        //self.settings.set_enable_mediastream(false);
        // self.settings.set_enable_mediasource(false);
        self.settings
            .set_enable_offline_web_application_cache(false);
        // self.settings.set_enable_private_browsing(true);
        // self.settings.set_enable_running_of_insecure_content(false);
        // self.settings.set_enable_display_of_insecure_content(false);
        self.settings.set_enable_xss_auditor(true);
        self.settings.set_media_playback_requires_user_gesture(true);
        self.settings.set_enable_developer_extras(true); // TODO: should only enabled conditionally

        self.parent_constructed(obj);

        self.setup_signals();
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.web_view.unparent();
    }
}
impl WidgetImpl for WebView {}

impl WebView {
    pub fn init_extension_message_receiver(&self) {
        let inst = self.instance().clone();

        let ctx = glib::MainContext::default();

        ctx.with_thread_default(move || {
            let ctx = glib::MainContext::default();

            ctx.spawn_local(async move {
                let this = Self::from_instance(&inst);

                // let listener = UnixListener::bind(&this.socket_path).await.unwrap();
                // let (socket, _addr) = listener.accept().await.unwrap();

                // let connection = Connection::new(socket);
                // this.connection.replace(Some(connection));

                let res = this.receive_extension_messages().await;
                debug!("receive_extension_messages result");
                // TODO: do something with this result...
            });
        });
    }

    pub async fn receive_extension_messages(&self) -> Result<(), ()> {
        dbg!("waiting for messages");

        loop {
            dbg!("waiting for message");

            let msg = self.connection.borrow_mut().next().await;
            if let Some(msg) = msg {
                self.process_message(&msg);
            } else {
                break;
            }
        }

        dbg!("stopped waiting for messages");
        Ok(())
    }

    pub fn process_message(&self, msg: &WebViewMessage) {
        dbg!("Received extension message: {:?}", msg);
        match msg {
            WebViewMessage::PreferredHeight(height) => {
                self.set_preferred_height(*height);
            }
            _ => {}
        };
    }

    pub fn set_preferred_height(&self, height: i64) {
        let inst = self.instance();
        dbg!("preferred size changed to: {:?}", height);

        inst.set_size_request(-1, height as i32);
        inst.queue_resize();
    }

    fn load_html(&self, _obj: &WebViewInstance, html: &str) {
        self.web_view.load_html(html, Some(INTERNAL_URL_BODY))
    }

    pub fn setup_signals(&self) {
        let inst = self.instance().clone();
        self.web_view.connect_load_changed(move |_, event| {
            let this = Self::from_instance(&inst);
            this.load_changed(event);
        });

        let inst = self.instance().clone();
        self.web_view
            .connect_decide_policy(move |_, decision, decision_type| {
                let this = Self::from_instance(&inst);
                this.decide_policy(decision, decision_type)
            });
    }

    pub fn load_changed(&self, event: webkit2gtk::LoadEvent) {
        info!("WebView: load changed: {:?}", event);

        match event {
            webkit2gtk::LoadEvent::Finished => {
                // self.init_extension_message_receiver();
                // if imp.client.is_ready() {
                //     self.ready_to_render();
                // }
            }
            _ => (),
        }
    }

    pub fn decide_policy(
        &self,
        decision: &webkit2gtk::PolicyDecision,
        decision_type: webkit2gtk::PolicyDecisionType,
    ) -> bool {
        debug!("webview: decide policy");

        match decision_type {
            // navigate to
            webkit2gtk::PolicyDecisionType::NavigationAction => {
                let navigation_decision: webkit2gtk::NavigationPolicyDecision = decision
                    .clone()
                    .downcast::<webkit2gtk::NavigationPolicyDecision>()
                    .unwrap();

                if navigation_decision.navigation_type() == webkit2gtk::NavigationType::LinkClicked
                {
                    decision.ignore();

                    // TODO: don't unwrap unconditionally
                    let uri = navigation_decision.request().unwrap().uri().unwrap();
                    info!("tv: navigating to: {}", uri);

                    let scheme = glib::uri_parse_scheme(&uri).unwrap();

                    match scheme.as_str() {
                        "mailto" => {
                            //uri = uri.substr (scheme.length ()+1, uri.length () - scheme.length()-1);
                            //           UstringUtils::trim(uri);
                            //           main_window->add_mode (new EditMessage (main_window, uri));
                        }
                        "id" | "mid" => {
                            //main_window->add_mode (new ThreadIndex (main_window, uri));
                        }
                        "http" | "https" | "ftp" => {
                            //open_link (uri);
                        }
                        _ => {
                            error!("tv: unknown uri scheme '{}'. not opening. ", scheme);
                        }
                    };
                }
            }
            _ => {
                decision.ignore();
            }
        };
        false
    }
}
