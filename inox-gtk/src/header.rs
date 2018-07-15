use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

use constants;

//use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    ThreadSelect(Option<String>)
}


pub struct HeaderModel {
    title: String
}

#[widget]
impl ::relm::Widget for Header {
    fn init_view(&mut self) {
        // self.label.set_text("Test");
    }

    fn model() -> HeaderModel {
        HeaderModel {
            title: constants::APPLICATION_NAME.to_string()
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::ThreadSelect(ref thread_id) => {
                println!("header: {:?}", thread_id.clone().unwrap());
                self.model.title = thread_id.clone().unwrap();
            }
        }
    }

    view! {
        #[name="container"]
        gtk::HeaderBar {
            title: Some(self.model.title.as_str()),
            show_close_button: true
        }
    }
}
