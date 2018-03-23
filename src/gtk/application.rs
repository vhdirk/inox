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



pub struct InoxApplication{
    win: gtk::ApplicationWindow,
    pub config_file: PathBuf,
    settings: Settings,

}

impl InoxApplication{

    pub fn new(gapp: &gtk::Application, config_path: &PathBuf) -> Rc<RefCell<Self>> {

        // load the settings
        let settings = Settings::new(&config_path.as_path());

        // open the notmuch database
        let db_ret = notmuch::Database::open(&settings.notmuch_config.database.path, notmuch::DatabaseMode::ReadWrite);

        match db_ret {
            Ok(db) => {
                debug!("opened db {:?}, revision {:?}", settings.notmuch_config.database.path, db.revision());

                let query = db.create_query(&"from:vhdirk@gmail.com".to_string()).unwrap();

                let mut threads = query.search_threads().unwrap();
                debug!("query {:?} {:?}  {:?} ", query, query.count_threads(), threads);

                let mut thread = threads.next().unwrap();
                debug!("thread {:?} {:?}", thread.subject(), thread.authors());

                let mut tags = db.all_tags().unwrap();
                debug!("tags {:?}", &tags.next());

            },
            Err(err) => {
                error!("db: failed to open database, please check the manual if everything is set up correctly: {:?}", err);
                // now quit.
            }
        }


        // Initialize UI.
        let builder = gtk::Builder::new_from_string(include_str!("main_window.ui"));
        let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();

        gapp.add_window(&window);



        // setup the the main application context

        let app = InoxApplication {
            win: window,
            config_file: config_path.to_path_buf(),
            settings: settings
        };


        let me = Rc::new(RefCell::new(app));

        // write the config back out.
        //me.borrow().config.store(&me.borrow().config_file);

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
