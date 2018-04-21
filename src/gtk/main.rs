#![feature(proc_macro)]
#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate shellexpand;
extern crate notmuch;

extern crate gtk;
extern crate gio;
#[macro_use]
extern crate glib;
extern crate gmime;
extern crate cairo;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk_sys as gtk_ffi;

extern crate inox_core;

use std::rc::Rc;
use std::cell::RefCell;
use std::fs::{File, DirBuilder};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gtk::prelude::*;
use gio::prelude::*;

use structopt::StructOpt;
use structopt::clap::{App, Arg};

use relm::Widget;

mod header;
mod constants;
mod main_content;
mod tag_list;
mod thread_list;
mod thread_list_cell_renderer;
mod thread_view;
mod application_window;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;
// use application::Application as InoxApplication;

use application_window::ApplicationWindow;


/// Init Gtk and logger.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        env_logger::init();

        // run initialization here
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }
    });
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(help="The configuration file to load.")]
    config: Option<PathBuf>,
    #[structopt(help="Print help message.")]
    help: bool,
}

impl Default for Args{
    fn default() -> Self {
        Args{
            config: Some(default_config_path()),
            help: false
        }
    }
}

fn default_config_path() -> PathBuf{
    let mut default_config = glib::get_user_config_dir().unwrap();
    default_config.push("inox");
    default_config.push("config");
    default_config.set_extension("toml");
    return default_config;
}

/// Main entry point
fn main() {
    init();

    let mut default_config = glib::get_user_config_dir().unwrap();
    default_config.push("inox");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap()).unwrap();

    // let args = Args::from_args();

    let args = App::new("Inox")
        .version("0.0.1")
        .author("Dirk Van Haerenborgh <vhdirk@gmail.com>")
        .about("A mail client with notmuch rust.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .default_value(default_config.to_str().unwrap())
                .help(
                    "The configuration file to load.",
                ),
        )
        .get_matches();

    let conf_location = args.value_of("config")
                        .unwrap_or(default_config.to_str().unwrap())
                        .to_string();

    debug!("Using config file {:?}", conf_location);

    // load the settings
    let conf_path:PathBuf = PathBuf::from(conf_location);

    let settings = Rc::new(Settings::new(&conf_path.as_path()));

    let dbman = Arc::new(DBManager::new(&settings));

    let gapp = gtk::Application::new(Some(constants::APPLICATION_ID),
                                     gio::ApplicationFlags::FLAGS_NONE).unwrap();

    gapp.connect_startup(move |app| {
        let mut _appwindow = ::relm::init::<ApplicationWindow>((app.to_owned(), settings.clone(), dbman.clone()));
    });
    gapp.connect_activate(|_| {

    });


    // Run GTK application with command line args
    let args: Vec<String> = std::env::args().collect();
    gapp.run(args.as_slice());
}
