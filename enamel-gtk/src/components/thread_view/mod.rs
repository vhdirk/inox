use std::rc::Rc;
use std::thread;
use std::process;
use std::cell::Cell;
use std::path::Path;
use serde_derive::{Serialize, Deserialize};
use log::*;
use gio;
use gio::SocketListenerExt;
use gio::SocketListenerExtManual;


use glib;
use gtk;
use gtk::prelude::*;
use webkit2gtk;
use webkit2gtk::{SettingsExt, WebViewExt, WebContextExt, PolicyDecisionExt, NavigationPolicyDecisionExt, URIRequestExt};
use std::sync::mpsc::{channel, Receiver};
use gmime;
use gmime::{ParserExt, PartExt};
use bincode;
use relm::{Relm, Widget, Update, connect, connect_stream, connect_async, connect_async_full};
use relm_derive::Msg;

use async_std::os::unix::net::{UnixStream, UnixListener};
use async_std::os::unix::io::IntoRawFd;

use uuid::Uuid;
use toml;

use notmuch;

use enamel_core::database::Thread;
use crate::app::EnamelApp;

mod page_client;
use page_client::{PageClient, Msg as PageClientMsg};


pub struct ThreadView{
    model: ThreadViewModel,
    container: gtk::Box,
    webview: webkit2gtk::WebView
}

pub struct ThreadViewModel {
    relm: Relm<ThreadView>,
    app: Rc<EnamelApp>,
    webcontext: webkit2gtk::WebContext,
    socket_listener: gio::SocketListener,
    page_client: Option<PageClient>
}




#[derive(Msg, Debug)]
pub enum Msg {
    InitializeWebExtensions,
    ExtensionConnect((gio::SocketConnection, Option<glib::Object>)),
    PageLoaded,

    LoadChanged(webkit2gtk::LoadEvent),
    ReadyToRender,
    DecidePolicy(webkit2gtk::PolicyDecision, webkit2gtk::PolicyDecisionType),
    
    ShowThread(Thread)
}


impl ThreadViewModel{

    fn initialize_web_extensions(ctx: &webkit2gtk::WebContext) -> gio::SocketListener
    {
        info!("initialize_web_extensions");

        let cur_exe = std::env::current_exe().unwrap();
        let exe_dir = cur_exe.parent().unwrap();
        let extdir = exe_dir.to_string_lossy(); 

        info!("setting web extensions directory: {:?}", extdir);
        ctx.set_web_extensions_directory(&extdir);

        let socket_addr = format!("/tmp/enamel-tv-{}-{}.sock", 
                                  process::id(),
                                  Uuid::new_v4());

        let gsock_addr = gio::UnixSocketAddress::new(Path::new(&socket_addr));

        let listener = gio::SocketListener::new();
        let res = listener.add_address(&gsock_addr,
                                       gio::SocketType::Stream,
                                       gio::SocketProtocol::Default,
                                       Some(&gsock_addr)).unwrap();
        info!("sock addr: {:?}", socket_addr);

        ctx.set_web_extensions_initialization_user_data(&socket_addr.to_variant());
        listener
    }
}


impl ThreadView{

    fn extension_connected(&mut self, conn: gio::SocketConnection, obj: Option<glib::Object>){
        debug!("ThreadView: extension_connected");
        let page_client = PageClient::new(conn);
        self.model.page_client = Some(page_client.clone());

        // TODO: create new macro to do this more elegantly
        // use self::PageClientMsg::PageLoaded as PageClient_PageLoaded;
        // let page_client_stream = page_client.stream.clone();
        // connect_stream!(page_client_stream@PageClient_PageLoaded, self.model.relm.stream(), Msg::PageLoaded);
    }

    fn load_changed(&mut self, event: webkit2gtk::LoadEvent){
        info!("ThreadView: load changed: {:?}", event);

        match event{
            webkit2gtk::LoadEvent::Finished => {
                if self.model.page_client.is_some(){
                    self.model.relm.stream().emit(Msg::ReadyToRender);
                    
                }
            },
            _ => ()
        }

    }

    fn ready_to_render(&mut self){

        match self.model.page_client.as_mut(){
            Some(pc) => {
                pc.load();

                /* render messages in case we were not ready when first requested */
                pc.clear_messages();
            },
            None => ()
        }
        
        self.render_messages();
    }

    // general message adding and rendering
    fn load_html(&self) {

        info!("render: loading html..");
        let _wk_loaded = false;
        let _ready = false;

        let html = gio::resources_lookup_data(&"/com/github/vhdirk/Enamel/html/thread_view.html", gio::ResourceLookupFlags::NONE).unwrap();
        let htmlcontent = std::str::from_utf8(&*html);

        self.webview.load_html(htmlcontent.unwrap(), None);

    }

    fn show_thread(&mut self, thread: Thread){


        debug!("Showing thread {:?}", thread);
        let messages = thread.messages();

        debug!("Showing thread {:?} > messages {:?}", thread, messages);
        for msg in messages{
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
                    if part.unwrap().is_attachment(){
                        debug!("Found attachment");
                    }
                }
                hasnext = partiter.next()
            }

        }
    }


    fn render_messages(&mut self){

    }


    fn decide_policy(&mut self, decision: &webkit2gtk::PolicyDecision, decision_type: webkit2gtk::PolicyDecisionType)
    {

        debug!("tv: decide policy");

        match decision_type {
            // navigate to
            webkit2gtk::PolicyDecisionType::NavigationAction => {

                let navigation_decision:webkit2gtk::NavigationPolicyDecision = decision.clone().downcast::<webkit2gtk::NavigationPolicyDecision>().unwrap();
                
                if navigation_decision.get_navigation_type() == webkit2gtk::NavigationType::LinkClicked{
                    decision.ignore();

                    // TODO: don't unwrap unconditionally
                    let uri = navigation_decision.get_request().unwrap().get_uri().unwrap();
                    info!("tv: navigating to: {}", uri);

                    let scheme = glib::uri_parse_scheme(&uri).unwrap();

                    match scheme.as_str() {
                        "mailto" => {
                            //uri = uri.substr (scheme.length ()+1, uri.length () - scheme.length()-1);
                            //           UstringUtils::trim(uri);
                            //           main_window->add_mode (new EditMessage (main_window, uri));
                        },
                        "id" | "mid" => {
                            //main_window->add_mode (new ThreadIndex (main_window, uri));
                        },
                        "http" | "https" | "ftp" => {
                            //open_link (uri);
                        },
                        _ => {
                            error!("tv: unknown uri scheme '{}'. not opening. ", scheme);
                        }

                    };

                }

            },
            _ => {
                decision.ignore();
            }
        };

  }

}


impl Update for ThreadView {
    type Model = ThreadViewModel;
    type ModelParam = Rc<EnamelApp>;
    type Msg = Msg;


    fn model(relm: &Relm<Self>, app: Self::ModelParam) -> Self::Model {
        let ctx = webkit2gtk::WebContext::get_default().unwrap();
        ctx.set_cache_model(webkit2gtk::CacheModel::DocumentViewer);

        // can't use relm for this since it would get called too late
        let listener = ThreadViewModel::initialize_web_extensions(&ctx);
        
        debug!("Starting connect");
        // accept connection from extension
        connect_async_full!(listener,
                            accept_async,
                            relm,
                            Msg::ExtensionConnect);
        ThreadViewModel {
            relm: relm.clone(),
            app,
            webcontext: ctx,
            socket_listener: listener,
            page_client: None
        }
    }


    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::InitializeWebExtensions => (), //self.initialize_web_extensions(),
            Msg::ExtensionConnect(result) => self.extension_connected(result.0, result.1),
            Msg::PageLoaded => (),
            Msg::LoadChanged(event) => self.load_changed(event), 
            Msg::ReadyToRender => self.ready_to_render(),
            Msg::DecidePolicy(decision, decision_type) => self.decide_policy(&decision, decision_type),
            Msg::ShowThread(thread) => self.show_thread(thread)
        }
    }
}

impl Widget for ThreadView {

    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.container.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self
    {
        let container = model.app.builder.get_object::<gtk::Box>("thread_view_box")
                                               .expect("Couldn't find thread_list_scrolled in ui file.");

        let ctx = model.webcontext.clone();

        let webview = webkit2gtk::WebView::new_with_context_and_user_content_manager(&ctx, &webkit2gtk::UserContentManager::new());

        container.pack_start(&webview, true, true, 0);
        
        ThreadView {
            model,
            container,
            webview
        }
    }


    fn init_view(&mut self) {
        
        info!("init view");

        let settings = webkit2gtk::WebViewExt::get_settings(&self.webview).unwrap();

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
 



        connect!(self.model.relm, self.webview, connect_load_changed(_,event), Msg::LoadChanged(event));

    // add_events (Gdk::KEY_PRESS_MASK);

        connect!(self.model.relm,
                 self.webview,
                 connect_decide_policy(_, decision, decision_type),
                 return (Msg::DecidePolicy(decision.clone(), decision_type), false));


        self.load_html();

    // register_keys ();


    }



}
