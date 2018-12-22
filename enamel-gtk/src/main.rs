use std::rc::Rc;
use std::path::PathBuf;
use std::fs::DirBuilder;

use log::*;
use env_logger;
use dirs;

#[cfg(test)]
use pretty_assertions;

use gtk;

use structopt::StructOpt;
use structopt::clap::{App, Arg};

mod macros;
mod static_resource;
mod constants;
mod app;
mod settings;
mod headerbar;
mod widgets;
mod components;
mod main_window;

use enamel_core::settings::Settings;

use crate::app::EnamelApp;

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
    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("enamel");
    default_config.push("config");
    default_config.set_extension("toml");
    default_config
}

/// Main entry point
fn main() {
    init();

    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("enamel");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap()).unwrap();

    // let args = Args::from_args();

    let args = App::new("Enamel")
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


    //
    // let gapp = InoxApplication::new(constants::APPLICATION_ID,
    //                                           gio::ApplicationFlags::empty())
    //                                      .expect("Initialization failed...");

    EnamelApp::run(settings);
}
