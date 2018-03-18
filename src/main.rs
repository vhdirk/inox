#[macro_use] extern crate clap;
use clap::{Arg, App};

#[macro_use] extern crate log;
extern crate log4rs;

extern crate regex;

use std::path::Path;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

extern crate gtk;
extern crate gio;
extern crate glib;
use gtk::prelude::*;

const CONFIG: &'static str = "./config/datalogserver.test.xml";


///
/// Initializes the logger so that it prints to stdout using log4rs
///
fn init_logger() {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(
            LogLevelFilter::Info,
        ))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
}

/// Main entry point
fn main() {
    if gtk::init().is_err(){
        error!("Failed to initialize GTK.");
        return;
    }

    let mut default_config = glib::get_user_config_dir().unwrap();
    default_config.push("some-mail");
    default_config.push("config");
    default_config.set_extension("toml");

    let matches = App::new("Some")
        .version("0.0.1")
        .author("Dirk Van Haerenborgh <vhdirk@gmail.com>")
        .about("A MUA based on notmuch. But does some more.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .default_value(default_config.to_str().unwrap())
                .help(
                    "The configuration file to load",
                ),
        )
        .get_matches();


    init_logger();



}
