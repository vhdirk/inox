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
use webkit2gtk::SettingsExt;

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
    scrolled_window: gtk::ScrolledWindow,
    webview: webkit2gtk::WebView

}

pub struct ThreadViewModel {
    relm: Relm<ThreadView>,
    app: Rc<EnamelApp>
}




#[derive(Msg, Debug)]
pub enum Msg {
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


    fn update(&mut self, _event: Msg) {
        // self.label.set_text("");
    }
}

impl Widget for ThreadView {

    type Root = gtk::ScrolledWindow;

    fn root(&self) -> Self::Root {
        self.scrolled_window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self
    {
        let scrolled_window = model.app.builder.get_object::<gtk::ScrolledWindow>("thread_list_scrolled")
                                               .expect("Couldn't find thread_list_scrolled in ui file.");


        let context = webkit2gtk::WebContext::get_default().unwrap();
        let webview = webkit2gtk::WebView::new_with_context(&context);

        ThreadView {
            model,
            scrolled_window,
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



    }



}
