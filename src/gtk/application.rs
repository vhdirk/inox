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

use header::Header;


pub struct Application {
    pub window: gtk::ApplicationWindow,
    pub header: Header,

    pub config_file: PathBuf,
    settings: Settings,

}

impl Application{

    pub fn new(gapp: &gtk::Application, config_path: &PathBuf) -> Self {

        // load the settings
        let settings = Settings::new(&config_path.as_path());

        // // open the notmuch database
        // let db_ret = notmuch::Database::open(&settings.notmuch_config.database.path, notmuch::DatabaseMode::ReadWrite);
        //
        // match db_ret {
        //     Ok(db) => {
        //         debug!("opened db {:?}, revision {:?}", settings.notmuch_config.database.path, db.revision());
        //
        //         let query = db.create_query(&"from:vhdirk@gmail.com".to_string()).unwrap();
        //
        //         let mut threads = query.search_threads().unwrap();
        //         debug!("query {:?} {:?}  {:?} ", query, query.count_threads(), threads);
        //
        //         let mut thread = threads.next().unwrap();
        //         debug!("thread {:?} {:?}", thread.subject(), thread.authors());
        //
        //         let mut tags = db.all_tags().unwrap();
        //         debug!("tags {:?}", &tags.next());
        //
        //     },
        //     Err(err) => {
        //         error!("db: failed to open database, please check the manual if everything is set up correctly: {:?}", err);
        //         // now quit.
        //     }
        // }
        //
        //
        // // Initialize UI.
        // let builder = gtk::Builder::new_from_string(include_str!("main_window.ui"));
        // //let window: gtk::ApplicationWindow::new(gapp);
        // let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
        //


        // Create a new top level window.
        let window = gtk::ApplicationWindow::new(gapp);
        // Create a the headerbar and it's associated content.
        let header = Header::new();

        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Set the title of the window.
        window.set_title("Inox");
        // Set the window manager class.
        window.set_wmclass("app-name", "Inox");
        // The icon the app will display.
        gtk::Window::set_default_icon_name("iconname");



        // Return our main application state
        Application {
            window,
            header,
            config_file: config_path.to_path_buf(),
            settings: settings
        }
    }

    pub fn connect_events(&mut self) {

        // Programs what to do when the exit button is used.
        self.window.connect_delete_event(move |_, _| {
          //gtk::main_quit();
          Inhibit(false)
        });


    }


    /// Start the app.
    pub fn start(&mut self) {




        // utils::setup_text_combo(&self.model_combo, &self.model_store);
        // utils::setup_text_combo(&self.port_combo, &self.port_store);
        // self.populate_model_combo();
        self.window.show_all();
    }


}
