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
pub enum HeaderMsg {
}


pub struct HeaderModel {
    
}

#[widget]
impl ::relm::Widget for Header {
    fn init_view(&mut self) {
        // self.label.set_text("Test");
    }

    fn model() -> HeaderModel {
        HeaderModel {

        }
    }

    fn update(&mut self, _event: HeaderMsg) {
        // self.label.set_text("");
    }

    view! {
        #[name="container"]
        gtk::HeaderBar {
            title: constants::APPLICATION_NAME,
            show_close_button: true
        }
    }
}
