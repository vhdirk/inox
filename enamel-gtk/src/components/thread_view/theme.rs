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
use webkit2gtk::{SettingsExt, WebViewExt};

use relm::init as relm_init;
use relm::{Relm, ToGlib, EventStream, Widget, Update};

use notmuch;
use notmuch::DatabaseMode;

use enamel_core::settings::Settings;
use enamel_core::database::Manager as DBManager;

use crate::app::EnamelApp;

type Thread = notmuch::Thread<'static, notmuch::Threads<'static, notmuch::Query<'static>>>;


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
    DecidePolicy(webkit2gtk::PolicyDecision, webkit2gtk::PolicyDecisionType)

}


impl ThreadView{

    fn render_messages(&self){

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
            Msg::DecidePolicy(decision, decision_type) => ()

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
        let webview = webkit2gtk::WebView::new_with_context(&context);

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
        settings.set_enable_developer_extras(true); // TODO: should only enabled conditionally

        self.webview.load_uri("https://crates.io/");

        connect!(self.model.relm, self.webview, connect_load_changed(_,event), Msg::LoadChanged(event));

    // add_events (Gdk::KEY_PRESS_MASK);

        connect!(self.model.relm, self.webview, connect_decide_policy(_,decision, decision_type),
                 return (Msg::DecidePolicy(decision.clone(), decision_type), false));


    // load_html ();

    // register_keys ();


    }



}
