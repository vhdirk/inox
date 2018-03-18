#![feature(custom_attribute)]

#[macro_use]
extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate regex;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::{File, DirBuilder};
use std::io::prelude::*;

use std::path::Path;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config as LogConfig, Logger, Root};

extern crate gtk;
extern crate gio;
extern crate glib;
use gtk::prelude::*;

mod config;
use config::Config;

///
/// Initializes the logger so that it prints to stdout using log4rs
///
fn init_logger() {
    let stdout = ConsoleAppender::builder().build();

    let config = LogConfig::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(
            LogLevelFilter::Info,
        ))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
}


/// Main entry point
fn main() {
    init_logger();

    if gtk::init().is_err(){
        error!("Failed to initialize GTK.");
        return;
    }

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
    let conf_path = Path::new(&conf_location);

    let conf: Config = Config::load(&conf_path);

    // write the config back out.
    conf.store(&conf_path);

    println!("config: {conf:?}", conf=conf);
}
