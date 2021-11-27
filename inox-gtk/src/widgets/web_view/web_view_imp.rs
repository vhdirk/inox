
use glib::subclass::prelude::*;
use std::cell::RefCell;

use log::*;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
use once_cell::unsync::OnceCell;

use std::os::unix::io::FromRawFd;
use gio::subclass::prelude::ObjectImplExt;
use glib::subclass::prelude::ClassStruct;
use glib::subclass::prelude::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use glib::Sender;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::WidgetImpl;
use gtk::subclass::widget::WidgetClassSubclassExt;
use webkit2gtk;
use webkit2gtk::traits::{
    NavigationPolicyDecisionExt, PolicyDecisionExt, SettingsExt, URIRequestExt, WebContextExt,
    WebViewExt as WebKitWebViewExt,
};

use crate::core::Action;
use crate::webextension::rpc::RawFdWrap;

use super::web_view_client::WebViewClient;
use super::theme::WebViewTheme;

pub type WebViewInstance = super::WebView;

/** URI Scheme and delimiter for internal resource loads. */
const INTERNAL_URL_PREFIX: &str = "inox:";

/** URI for internal message body page loads. */
// TODO: create from INTERNAL_URL_PREFIX
const INTERNAL_URL_BODY: &str = "inox:body";

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


#[derive(Clone, Debug)]
pub struct WebView {
    pub web_view: webkit2gtk::WebView,
    pub web_context: webkit2gtk::WebContext,
    pub settings: webkit2gtk::Settings,
    pub page_client: WebViewClient,
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
        let stream = initialize_web_extensions(&web_context);
        let page_client = WebViewClient::new(&stream);

        let settings = WebKitWebViewExt::settings(&web_view).unwrap();

        WebView {
            web_view,
            web_context,
            settings,
            page_client,
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

    fn load_html(&self, _obj: &WebViewInstance, html: &str) {
        self.web_view.load_html(html, Some(INTERNAL_URL_BODY))
    }

    pub fn setup_signals(&self) {
        let self_ = self.clone();
        self.web_view
            .connect_decide_policy(move |_, decision, decision_type| {
                let mut mself = self_.clone();
                mself.decide_policy(decision, decision_type);
                false
            });
    }

    pub fn decide_policy(
        &mut self,
        decision: &webkit2gtk::PolicyDecision,
        decision_type: webkit2gtk::PolicyDecisionType,
    ) {
        debug!("tv: decide policy");

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
    }
}
