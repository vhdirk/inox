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

#[widget]
impl ::relm::Widget for ThreadView {

    fn model() -> ThreadViewModel {
        ThreadViewModel {

        }
    }

    fn update(&mut self, _event: ThreadViewMsg) {
        // self.label.set_text("");
    }

    view! {
        gtk::Box{

        }
    }
}
