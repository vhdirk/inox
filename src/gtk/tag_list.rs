use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use notmuch;

use inox_core::settings::Settings;


pub struct TagList {
    pub container: gtk::ListBox,

}



impl TagList {
    pub fn new() -> Self {

        let container = gtk::ListBox::new();

        TagList { container }
    }
}
