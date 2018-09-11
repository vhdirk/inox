use gio::ActionMapExt;
use gtk::GtkWindowExt;
use gtk;
use gio;
use glib;
use gtk::prelude::*;

use crossbeam_channel::Sender;
use failure::Error;
use rayon;
// use url::Url;

// use hammond_data::{dbqueries, Source};

use app::EnamelApp;
use controller::Action;
use stacks::Content;
// use utils::{itunes_to_rss, refresh};

use std::rc::Rc;

#[derive(Debug, Clone)]
// TODO: Factor out the hamburger menu
// TODO: Make a proper state machine for the headerbar states
pub struct MainWindow {
    pub(crate) container: gtk::ApplicationWindow,
}

impl Default for MainWindow {
    fn default() -> MainWindow {
        let builder = gtk::Builder::new_from_resource("/com/github/vhdirk/Enamel/gtk/main_window.ui");

        let container = builder.get_object("main_window").unwrap();

        MainWindow{
            container
        }
    }
}

impl MainWindow {
    pub(crate) fn new(sender: &Sender<Action>) -> Rc<Self> {
        let window = Rc::new(MainWindow::default());
        Self::init(&window, &sender);
        window
    }

    pub(crate) fn init(this: &Rc<Self>, sender: &Sender<Action>) {
        let weak = Rc::downgrade(this);

        //self.switch.set_stack(&content.get_stack());
    }
}