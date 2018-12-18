use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;
use webkit2gtk;
use webkit2gtk::{SettingsExt, WebViewExt, WebContextExt, PolicyDecisionExt, NavigationPolicyDecisionExt, URIRequestExt};
use gmime;
use gmime::{ParserExt, PartExt};

use relm::init as relm_init;
use relm::{Relm, ToGlib, EventStream, Widget, Update};

use notmuch;
use notmuch::{DatabaseMode};

use enamel_core::settings::Settings;
use enamel_core::database::Manager as DBManager;

use crate::app::EnamelApp;

type Thread = notmuch::Thread<'static, 'static>;


pub struct ThreadView{
    model: ThreadViewModel,
    container: gtk::Box,
    webview: webkit2gtk::WebView
}

pub struct ThreadViewModel {
    relm: Relm<ThreadView>,
    app: Rc<EnamelApp>
}




#[derive(Msg, Debug)]
pub enum Msg {
    LoadChanged(webkit2gtk::LoadEvent),
    DecidePolicy(webkit2gtk::PolicyDecision, webkit2gtk::PolicyDecisionType),
    
    ShowThread(Rc<Thread>)
}


impl ThreadView{

    // general message adding and rendering
    fn load_html(&self) {
        info!("render: loading html..");
        let wk_loaded = false;
        let ready = false;

        let html = gio::resources_lookup_data(&"/com/github/vhdirk/Enamel/html/thread_view.html", gio::ResourceLookupFlags::NONE).unwrap();
        let htmlcontent = std::str::from_utf8(&*html);

        self.webview.load_html(htmlcontent.unwrap(), None);

    }

    fn show_thread(&mut self, thread: Rc<Thread>){
        debug!("Showing thread {:?}", thread);
        let mut messages = thread.messages();

        debug!("Showing thread {:?} > messages {:?}", thread, messages);
        while let Some(msg) = messages.next(){
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
        ThreadViewModel {
            relm: relm.clone(),
            app
        }
    }


    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::LoadChanged(event) => (), 
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

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self
    {
        let container = model.app.builder.get_object::<gtk::Box>("thread_view_box")
                                               .expect("Couldn't find thread_list_scrolled in ui file.");


        let context = webkit2gtk::WebContext::get_default().unwrap();
        let webview = webkit2gtk::WebView::new_with_context_and_user_content_manager(&context, &webkit2gtk::UserContentManager::new());

        container.pack_start(&webview, true, true, 0);


        ThreadView {
            model,
            container,
            webview
        }
    }


    fn init_view(&mut self) {

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
        settings.set_enable_page_cache(false);
        settings.set_enable_private_browsing(true);
        // settings.set_enable_running_of_insecure_content(false);
        // settings.set_enable_display_of_insecure_content(false);
        settings.set_enable_xss_auditor(true);
        settings.set_media_playback_requires_user_gesture(true);
        settings.set_enable_developer_extras(true   ); // TODO: should only enabled conditionally
 
        connect!(self.model.relm, self.webview, connect_load_changed(_,event), Msg::LoadChanged(event));

    // add_events (Gdk::KEY_PRESS_MASK);

        connect!(self.model.relm, self.webview, connect_decide_policy(_,decision, decision_type),
                 return (Msg::DecidePolicy(decision.clone(), decision_type), false));


        let ctx = self.webview.get_context().unwrap();
        ctx.set_web_extensions_directory("/home/dvhaeren/projects/enamel/target/debug");

        self.load_html();

    // register_keys ();


    }



}
