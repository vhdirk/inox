use async_std::os::unix::net::UnixStream;
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
use gmime::{ParserExt, PartExt};
use gtk;
use gtk::prelude::*;
use webkit2gtk;
use webkit2gtk::{
    NavigationPolicyDecisionExt, PolicyDecisionExt, SettingsExt, URIRequestExt, WebContextExt,
    WebViewExt,
};

use crate::app::Action;
use crate::spawn;
use crate::webextension::rpc::RawFdWrap;
use inox_core::database::Thread;

mod page_client;
use page_client::PageClient;
mod theme;
use theme::ThreadViewTheme;

#[derive(Clone)]
pub struct ThreadView {
    pub widget: gtk::Box,
    sender: Sender<Action>,
    webview: webkit2gtk::WebView,

    webcontext: webkit2gtk::WebContext,
    page_client: PageClient,
    theme: ThreadViewTheme,
    thread: RefCell<Option<Thread>>,
}

impl ThreadView {
    pub fn new(sender: Sender<Action>) -> Self {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let webcontext = webkit2gtk::WebContext::get_default().unwrap();
        webcontext.set_cache_model(webkit2gtk::CacheModel::DocumentViewer);

        let stream = ThreadView::initialize_web_extensions(&webcontext);
        let page_client = PageClient::new(stream);

        let webview = webkit2gtk::WebView::new_with_context_and_user_content_manager(
            &webcontext,
            &webkit2gtk::UserContentManager::new(),
        );

        widget.pack_start(&webview, true, true, 0);

        let settings = webkit2gtk::WebViewExt::get_settings(&webview).unwrap();

        // settings.set_enable_scripts(true);
        // settings.set_enable_java_applet(false);
        settings.set_enable_plugins(false);
        settings.set_auto_load_images(true);
        settings.set_enable_dns_prefetching(false);
        settings.set_enable_fullscreen(false);
        settings.set_enable_html5_database(false);
        settings.set_enable_html5_local_storage(false);
        //settings.set_enable_mediastream(false);
        // settings.set_enable_mediasource(false);
        settings.set_enable_offline_web_application_cache(false);
        // settings.set_enable_private_browsing(true);
        // settings.set_enable_running_of_insecure_content(false);
        // settings.set_enable_display_of_insecure_content(false);
        settings.set_enable_xss_auditor(true);
        settings.set_media_playback_requires_user_gesture(true);
        settings.set_enable_developer_extras(true); // TODO: should only enabled conditionally

        ThreadView {
            widget,
            sender,
            webview,
            webcontext,
            page_client: page_client,
            theme: ThreadViewTheme::load(),
            thread: RefCell::new(None),
        }
    }

    pub fn setup_signals(&self) {
        let self_ = self.clone();
        self.webview.connect_load_changed(move |_, event| {
            let mut mself = self_.clone();

            mself.load_changed(event);
        });

        //     // add_events (Gdk::KEY_PRESS_MASK);
        let self_ = self.clone();
        self.webview
            .connect_decide_policy(move |_, decision, decision_type| {
                let mut mself = self_.clone();
                mself.decide_policy(decision, decision_type);
                false
            });

        self.load_html();

        //     // register_keys ();
    }

    fn load_changed(&mut self, event: webkit2gtk::LoadEvent) {
        info!("ThreadView: load changed: {:?}", event);

        match event {
            webkit2gtk::LoadEvent::Finished => {
                if self.page_client.is_ready() {
                    self.ready_to_render();
                }
            }
            _ => (),
        }
    }

    async fn ready_to_render(&mut self) {
        info!("ready_to_render");

        self.page_client.load(&self.theme).await;

        /* render messages in case we were not ready when first requested */
        self.page_client.clear_messages().await;

        // self.render_messages().await;
    }

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
            SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        )
        .unwrap();

        ctx.set_web_extensions_initialization_user_data(&remote.to_variant());

        unsafe { gio::Socket::new_from_fd(RawFdWrap::from_raw_fd(local)) }.unwrap()
    }

    // general message adding and rendering
    fn load_html(&self) {
        info!("render: loading html..");

        self.webview.load_html(&self.theme.html, None);
    }

    pub fn load_thread(&self, thread: Thread) {
        info!("load_thread: {:?}", thread);

        let mut client = self.page_client.clone();
        let mut self_ = self.clone();

        let future = async move {
            debug!("clearing messages");
            client.clear_messages().await;

            debug!("render messages");
            self_.render_messages(thread).await
        };

        spawn!(future);
    }

    pub fn show_thread(&self, thread: Thread) {
        debug!("Showing thread {:?}", thread);
        let messages = thread.messages();

        debug!("Showing thread {:?} > messages {:?}", thread, messages);
        for msg in messages {
            let fname = msg.filename();
            info!("message: {:?}", fname);

            let stream = gmime::StreamFile::open(&fname.to_string_lossy(), &"r").unwrap();
            let parser = gmime::Parser::new_with_stream(&stream);
            let mmsg = parser.construct_message(None);

            info!("created mime message: {:?}", mmsg);

            let mut partiter = gmime::PartIter::new(&mmsg.unwrap());

            let mut hasnext = partiter.next();
            while hasnext {
                let current = partiter.get_current().unwrap();
                let parent = partiter.get_parent().unwrap();

                let p = parent.downcast::<gmime::Multipart>();
                let part = current.downcast::<gmime::Part>();

                if p.is_ok() && part.is_ok() {
                    if part.unwrap().is_attachment() {
                        debug!("Found attachment");
                    }
                }
                hasnext = partiter.next()
            }
        }
    }

    async fn add_message(&mut self, message: &notmuch::Message<'_, notmuch::Thread<'_, '_>>) {
        let mut client = self.page_client.clone();

        client.add_message(message);
    }

    async fn render_messages(&mut self, thread: Thread) {
        debug!("render: html loaded, building messages..");

        // for message in thread.messages() {
        //     self.add_message(&message).await;
        // }

        // /* set message state vector */
        // state.clear ();
        // focused_message.clear ();

        // if (mthread) {
        //   for (auto &m : mthread->messages) {
        //     add_message (m);
        //   }

        //   page_client->update_state ();
        //   update_all_indent_states ();

        //   /* focus oldest unread message */
        //   if (!edit_mode) {
        //     for (auto &m : mthread->messages_by_time ()) {
        //       if (m->has_tag ("unread")) {
        //         focused_message = m;
        //         break;
        //       }
        //     }
        //   }

        //   if (!focused_message) {
        //     LOG (debug) << "tv: no message focused, focusing newest message.";
        //     focused_message = *max_element (
        //         mthread->messages.begin (),
        //         mthread->messages.end (),
        //         [](refptr<Message> &a, refptr<Message> &b)
        //           {
        //             return ( a->time < b->time );
        //           });
        //   }

        //   expand (focused_message);
        //   focus_message (focused_message);

        //   ready = true;
        //   emit_ready ();

        //   if (!edit_mode && !unread_setup) {
        //     unread_setup = true;

        //     if (unread_delay > 0) {
        //       Glib::signal_timeout ().connect (
        //           sigc::mem_fun (this, &ThreadView::unread_check), std::max (80., (unread_delay * 1000.) / 2));
        //     } else {
        //       unread_check ();
        //     }
        //   }
        // } else {
        //   LOG (debug) << "tv: no message thread.";
        // }
    }

    fn decide_policy(
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

                if navigation_decision.get_navigation_type()
                    == webkit2gtk::NavigationType::LinkClicked
                {
                    decision.ignore();

                    // TODO: don't unwrap unconditionally
                    let uri = navigation_decision
                        .get_request()
                        .unwrap()
                        .get_uri()
                        .unwrap();
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

// impl Update for ThreadView {
//     type Model = ThreadViewModel;
//     type ModelParam = Rc<EnamelApp>;
//     type Msg = Msg;

//     fn model(relm: &Relm<Self>, app: Self::ModelParam) -> Self::Model {
//         let ctx = webkit2gtk::WebContext::get_default().unwrap();
//         ctx.set_cache_model(webkit2gtk::CacheModel::DocumentViewer);

//         // can't use relm for this since it would get called too late
//         let listener = ThreadViewModel::initialize_web_extensions(&ctx);

//         debug!("Starting connect");
//         // accept connection from extension
//         connect_async_full!(listener,
//                             accept_async,
//                             relm,
//                             Msg::ExtensionConnect);
//         ThreadViewModel {
//             relm: relm.clone(),
//             app,
//             webcontext: ctx,
//             socket_listener: listener,
//             page_client: None,
//             theme: ThreadViewTheme::load()
//         }
//     }

//     fn update(&mut self, msg: Msg) {
//         match msg {
//             Msg::InitializeWebExtensions => (), //self.initialize_web_extensions(),
//             Msg::ExtensionConnect(result) => self.extension_connected(result.0, result.1),
//             Msg::PageLoaded => (),
//             Msg::LoadChanged(event) => self.load_changed(event),
//             Msg::ReadyToRender => self.ready_to_render(),
//             Msg::DecidePolicy(decision, decision_type) => self.decide_policy(&decision, decision_type),
//             Msg::ShowThread(thread) => self.show_thread(thread)
//         }
//     }
// }

// impl Widget for ThreadView {

//     type Root = gtk::Box;

//     fn root(&self) -> Self::Root {
//         self.container.clone()
//     }

//     fn view(_relm: &Relm<Self>, model: Self::Model) -> Self
//     {

//     }

//     fn init_view(&mut self) {

//         info!("init view");

//         let settings = webkit2gtk::WebViewExt::get_settings(&self.webview).unwrap();

//         // settings.set_enable_scripts(true);
//         // settings.set_enable_java_applet(false);
//         settings.set_enable_plugins(false);
//         settings.set_auto_load_images(true);
//         settings.set_enable_dns_prefetching(false);
//         settings.set_enable_fullscreen(false);
//         settings.set_enable_html5_database(false);
//         settings.set_enable_html5_local_storage(false);
//         //settings.set_enable_mediastream(false);
//         // settings.set_enable_mediasource(false);
//         settings.set_enable_offline_web_application_cache(false);
//         // settings.set_enable_private_browsing(true);
//         // settings.set_enable_running_of_insecure_content(false);
//         // settings.set_enable_display_of_insecure_content(false);
//         settings.set_enable_xss_auditor(true);
//         settings.set_media_playback_requires_user_gesture(true);
//         settings.set_enable_developer_extras(true); // TODO: should only enabled conditionally

//         connect!(self.model.relm, self.webview, connect_load_changed(_,event), Msg::LoadChanged(event));

//     // add_events (Gdk::KEY_PRESS_MASK);

//         connect!(self.model.relm,
//                  self.webview,
//                  connect_decide_policy(_, decision, decision_type),
//                  return (Msg::DecidePolicy(decision.clone(), decision_type), false));

//         self.load_html();

//     // register_keys ();

//     }

// }
