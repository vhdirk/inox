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
use webkit2gtk::prelude::*;
use webkit2gtk::SettingsExt;

use relm_attributes::widget;

use notmuch;

use inox_core::settings::Settings;


// pub struct ThreadView {
//     pub container: gtk::ListBox,
//
// }
//
//
//
// impl ThreadView {
//     pub fn new() -> Self {
//
//         let container = gtk::ListBox::new();
//
//         ThreadView { container }
//     }
// }


#[derive(Msg)]
pub enum ThreadViewMsg {
}


pub struct ThreadViewModel {

}

impl Widget for ThreadView {

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
        settings.set_enable_mediasource(false);
        settings.set_enable_offline_web_application_cache(false);
        settings.set_enable_page_cache(false);
        settings.set_enable_private_browsing(true);
        // settings.set_enable_running_of_insecure_content(false);
        // settings.set_enable_display_of_insecure_content(false);
        settings.set_enable_xss_auditor(true);
        settings.set_media_playback_requires_user_gesture(true);
        settings.set_enable_developer_extras(true); // TODO: should only enabled conditionally



    }


    fn model() -> ThreadViewModel {
        ThreadViewModel {

        }
    }

    fn update(&mut self, _event: ThreadViewMsg) {
        // self.label.set_text("");
    }
}
