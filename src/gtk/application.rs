use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use notmuch;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use header::Header;
use constants;
use main_content::MainContent;

pub struct Application {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub content: MainContent,

    settings: Rc<Settings>,
    dbmanager: Rc<DBManager>

}

impl Application{

    pub fn new(gapp: &gtk::Application, settings: Rc<Settings>, dbmanager: Rc<DBManager>) -> Self {


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

        window.set_default_size(800, 600);

        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Set the title of the window.
        window.set_title(constants::APPLICATION_NAME);
        // Set the window manager class.
        window.set_wmclass(constants::APPLICATION_CLASS, constants::APPLICATION_NAME);
        // The icon the app will display.
        gtk::Window::set_default_icon_name(constants::APPLICATION_ICON_NAME);

        // Create the content container and all of it's widgets.
        let content = MainContent::new(dbmanager.clone());

        // Add the content to the window.
        window.add(&content.container);

        // Return our main application state
        Application {
            window,
            header,
            content,
            settings: settings.clone(),
            dbmanager: dbmanager.clone()
        }
    }

    pub fn connect_events(&mut self) {
        // Keep track of whether we are fullscreened or not.
        let fullscreen = Arc::new(AtomicBool::new(false));

        // Programs what to do when the exit button is used.
        self.window.connect_delete_event(move |_, _| {
          //gtk::main_quit();
          Inhibit(false)
        });


    }

    /// Handles special functions that should be invoked when certain keys and key combinations
    /// are pressed on the keyboard.
    // fn key_events(
    //     &self,
    //     current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    //     fullscreen: Arc<AtomicBool>,
    // ) {
    //     // Grab required references beforehand.
    //     let editor = self.content.source.buff.clone();
    //     let headerbar = self.header.container.clone();
    //     let save_button = self.header.save.clone();
    //
    //     // Each key press will invoke this function.
    //     self.window.connect_key_press_event(move |window, gdk| {
    //         match gdk.get_keyval() {
    //             // Fullscreen the UI when F11 is pressed.
    //             key::F11 => if fullscreen.fetch_xor(true, Ordering::SeqCst) {
    //                 window.unfullscreen();
    //             } else {
    //                 window.fullscreen();
    //             },
    //             // Save the file when ctrl+s is pressed.
    //             key if key == 's' as u32 && gdk.get_state().contains(CONTROL_MASK) => {
    //                 save(&editor, &headerbar, &save_button, &current_file, false);
    //             }
    //             _ => (),
    //         }
    //         Inhibit(false)
    //     });
    // }

    /// Start the app.
    pub fn start(&mut self) {




        // utils::setup_text_combo(&self.model_combo, &self.model_store);
        // utils::setup_text_combo(&self.port_combo, &self.port_store);
        // self.populate_model_combo();
        self.window.show_all();
    }


}
