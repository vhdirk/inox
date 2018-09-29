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

extern crate shellexpand;
extern crate notmuch;
extern crate chrono;
extern crate crossbeam_channel;
extern crate rayon;
extern crate md5;

#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;
#[macro_use]
extern crate glib;
extern crate gmime;
extern crate cairo;
extern crate pango;
extern crate pangocairo;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate cairo_sys as cairo_ffi;
extern crate gdk_sys as gdk_ffi;

// extern crate webkit2gtk;

#[macro_use]
extern crate gobject_subclass;
#[macro_use]
extern crate gio_subclass;
#[macro_use]
extern crate gtk_subclass;

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate enamel_core;


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

#[macro_use]
mod macros;
mod static_resource;
mod constants;
mod app;
mod settings;
mod headerbar;
mod widgets;
mod main_window;

// mod main_content;
// mod tag_list;
// mod thread_list;
// mod cell_renderer;
// mod thread_list_cell_renderer;
// mod thread_view;
// mod application_window;
// mod util;

// mod application;

use enamel_core::settings::Settings;
use enamel_core::database::Manager as DBManager;
// use application::Application as InoxApplication;

// use application_window::ApplicationWindow;
use app::EnamelApp;

/// Init Gtk and logger.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        env_logger::init();

        // run initialization here
        gtk::init().expect("Error initializing gtk.");
        static_resource::init().expect("Error initializing static resources.");
    });
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(help="The configuration file to load.", parse(from_os_str))]
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
    default_config.push("enamel");
    default_config.push("config");
    default_config.set_extension("toml");
    return default_config;
}

/// Main entry point
fn main() {
    init();

    let mut default_config = glib::get_user_config_dir().unwrap();
    default_config.push("enamel");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap()).unwrap();

    // let args = Args::from_args();

    let args = App::new("Inox")
        .version("0.0.1")
        .author("Dirk Van Haerenborgh <vhdirk@gmail.com>")
        .about("An email client with notmuch rust.")
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

    //let dbman = Rc::new(DBManager::new(&settings));

    //
    // let gapp = InoxApplication::new(constants::APPLICATION_ID,
    //                                           gio::ApplicationFlags::empty())
    //                                      .expect("Initialization failed...");

    EnamelApp::run(settings);
}
