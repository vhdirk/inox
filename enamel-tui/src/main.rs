use structopt;

use log;
use env_logger;

use serde;
use serde_derive;

use std::rc::Rc;
use std::cell::RefCell;
use std::io;
use std::fs::DirBuilder;
use std::path::PathBuf;

use dirs;
use log::*;

use structopt::StructOpt;
use structopt::clap::{App, Arg};

use cursive::Cursive;
use cursive::views::{Dialog, TextView, ListView};

use enamel_core::settings::Settings;

mod app;
mod list_view;
mod lazy_list_view;
mod thread_list;

use app::EnamelApp;



/// Init logger.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        env_logger::init();
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
    return default_config;
}


// const COMMANDS = {
//     "search": {},
//     "envelope": {},
//     "bufferlist": {},
//     "taglist": {},
//     "namedqueries": {},
//     "thread": {},
//     "global": {},
// };


// pub trait Command {
//     fn apply() {}
//     fn pre_hooks() {}
//     fn post_hooks() {}
// }

fn main() -> Result<(), io::Error> {
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


    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();


    // Create the app
    let mut app = EnamelApp::new(&mut siv, settings);

    // Starts the event loop.
    siv.run();

    Ok(())
}