#![feature(custom_derive)]
#![feature(custom_attribute)]

use std::rc::Rc;
use std::cell::RefCell;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate regex;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate toml;

extern crate shellexpand;

use std::fs::{File, DirBuilder};
use std::io::prelude::*;

use std::path::{Path, PathBuf};

extern crate gtk;
extern crate gio;
extern crate glib;
use gtk::prelude::*;
use gio::prelude::*;

extern crate some_core;

mod application;
use application::SomeApplication;

pub const GTK_APPLICATION_ID: &'static str = "com.github.vhdirk.somemail";


/// Init Gtk and stuff.
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


/// Main entry point
fn main() {
    init();

    let mut default_config = glib::get_user_config_dir().unwrap();
    default_config.push("some-mail");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap()).unwrap();

    default_config.push("config");
    default_config.set_extension("toml");

    let args = App::new("Some")
        .version("0.0.1")
        .author("Dirk Van Haerenborgh <vhdirk@gmail.com>")
        .about("A MUA based on notmuch. But does some more.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .default_value(default_config.to_str().unwrap())
                .help(
                    "The configuration file to load. Will write the default config to this file if it does not exist.",
                ),
        )
        .get_matches();



    let conf_location = args.value_of("config")
                        .unwrap_or(default_config.to_str().unwrap())
                        .to_string();

    debug!("Using config file {:?}", conf_location);

    let gapp = gtk::Application::new(Some(GTK_APPLICATION_ID),
                                     gio::ApplicationFlags::FLAGS_NONE).unwrap();

    gapp.connect_activate(move |gapp| {
        let conf_path:PathBuf = PathBuf::from(conf_location.to_owned());

        let app = SomeApplication::new(&gapp, &conf_path);
        app.borrow_mut().start();
    });

    // Run GTK application with command line args
    let args: Vec<String> = std::env::args().collect();
    gapp.run(args.as_slice());
}
