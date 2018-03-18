use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use some_core::config::Config;



pub struct SomeApplication{
    win: gtk::ApplicationWindow,
    pub config_file: PathBuf,
    config: Config,

}

impl SomeApplication{
    pub fn new(gapp: &gtk::Application, config_path: &PathBuf) -> Rc<RefCell<Self>> {
        let builder = gtk::Builder::new_from_string(include_str!("somewindow.ui"));
        let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();


        gapp.add_window(&window);

        let app = SomeApplication {
            win: window,
            config_file: config_path.to_path_buf(),
            config: Config::load(&config_path.as_path())
        };

        let me = Rc::new(RefCell::new(app));

        // write the config back out.
        me.borrow().config.store(&me.borrow().config_file);

        return me;
    }

    /// Start the app.
    pub fn start(&mut self) {

        // utils::setup_text_combo(&self.model_combo, &self.model_store);
        // utils::setup_text_combo(&self.port_combo, &self.port_store);
        // self.populate_model_combo();
        self.win.show_all();
    }


}
