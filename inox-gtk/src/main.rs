use std::fs::DirBuilder;
use std::path::PathBuf;
use std::rc::Rc;

use dirs;
use log::*;
use pretty_env_logger;

use structopt::clap::{App, Arg};
use structopt::StructOpt;
mod application;
mod constants;
mod macros;
mod settings;
mod static_resource;
mod core;
mod widgets;

mod webextension;

use inox_core::settings::Settings;

use crate::application::InoxApplication;

pub mod webext_capnp {
    include!(concat!(env!("OUT_DIR"), "/resources/webext_capnp.rs"));
}

/// Init Gtk and logger.
fn init() {
    use std::sync::Once;

    static START: Once = Once::new();

    START.call_once(|| {
        pretty_env_logger::init();

        // run initialization here
        gtk::init().expect("Error initializing gtk.");
        static_resource::init().expect("Error initializing static resources.");
    });
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(help = "The configuration file to load.", parse(from_os_str))]
    config: Option<PathBuf>,
    #[structopt(help = "Print help message.")]
    help: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            config: Some(default_config_path()),
            help: false,
        }
    }
}

fn default_config_path() -> PathBuf {
    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("inox");
    default_config.push("config");
    default_config.set_extension("toml");
    default_config
}

fn main() {
    init();

    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("inox");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap())
        .unwrap();

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
                .help("The configuration file to load."),
        )
        .get_matches();

    let conf_location = args
        .value_of("config")
        .unwrap_or(default_config.to_str().unwrap())
        .to_string();

    debug!("Using config file {:?}", conf_location);

    // load the settings
    let conf_path: PathBuf = PathBuf::from(conf_location);
    let settings = Rc::new(Settings::new(&conf_path.as_path()));

    // Initialize variables
    glib::set_application_name(constants::APPLICATION_NAME);
    glib::set_prgname(Some(&constants::APPLICATION_NAME));
    gtk::Window::set_default_icon_name(constants::APPLICATION_ICON_NAME);
    // env::set_var("PULSE_PROP_application.icon_name", constants::APPLICATION_ID);
    // env::set_var("PULSE_PROP_application.name", constants::APPLICATION_NAME);

    // Setup translations
    // setlocale(LocaleCategory::LcAll, "");
    // bindtextdomain(constants::PKGNAME, constants::LOCALEDIR);
    // textdomain(constants::PKGNAME);

    let ctx = glib::MainContext::default();
    let _guard = ctx.acquire().unwrap();
    // Run app itself
    InoxApplication::run(settings);
}
