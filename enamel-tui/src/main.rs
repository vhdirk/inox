use structopt;

use log;
use env_logger;

use serde;
use serde_derive;


use std::io;
use std::fs::DirBuilder;
use std::path::PathBuf;

use dirs;

use structopt::StructOpt;
use structopt::clap::{App, Arg};

use cursive::Cursive;
use cursive::views::{Dialog, TextView};


struct EnamelApp {
    // size: Rect,
}

impl Default for EnamelApp {
    fn default() -> EnamelApp {
        EnamelApp {
            // size: Rect::default(),
        }
    }
}

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
    let _args = App::new("Enamel")
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


    let initial_command = "search tag:inbox AND NOT tag:killed";

    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
                         .title("Cursive")
                         .button("Quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();

    Ok(())
}