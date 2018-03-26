use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;


pub struct Header {
    pub container: gtk::HeaderBar
}


impl Header {
    pub fn new() -> Header {
        // Creates the main header bar container widget.
        let container = gtk::HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title("App Name");
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Returns the header and all of it's state
        Header {
            container
        }
    }
}
