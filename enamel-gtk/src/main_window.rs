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
use uibuilder::UI;
use app::Action;
use stacks::Content;
// use utils::{itunes_to_rss, refresh};

use std::rc::Rc;

#[derive(Debug, Clone)]
// TODO: Factor out the hamburger menu
// TODO: Make a proper state machine for the headerbar states
pub struct MainWindow {
    ui: UI,
    pub(crate) container: gtk::ApplicationWindow,
}

impl MainWindow {
    pub(crate) fn new(ui: UI, 
                      application: gtk::Application) -> Rc<Self> {
        let window = Rc::new(MainWindow{
            ui: ui.clone(),
            container: ui.builder
                .get_object("main_window")
                .expect("Couldn't find main_window in ui file.")
        });
        window.container.set_application(&application);
        Self::init(&window);
        window
    }

    pub(crate) fn init(this: &Rc<Self>/*, sender: &Sender<Action>*/) {
        let weak = Rc::downgrade(this);

        //self.switch.set_stack(&content.get_stack());
    }
}