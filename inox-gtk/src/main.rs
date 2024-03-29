use std::fs::DirBuilder;
use std::path::PathBuf;
use std::rc::Rc;

use dirs;
use log::*;
use pretty_env_logger;
use gmime;

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

/// Init Gtk and logger.
fn init() {
    use std::sync::Once;

    static START: Once = Once::new();

    START.call_once(|| {
        pretty_env_logger::init();

        // run initialization here
        gtk::init().expect("Error initializing gtk.");
        static_resource::init().expect("Error initializing static resources.");
        gmime::functions::init();
    });
}

#[derive(Debug, StructOpt)]
#[structopt(name="inox-gtk", about = "An email client with notmuch rust.", author="Dirk Van Haerenborgh <vhdirk@gmail.com>", version="0.0.1")]
struct Args {
    #[structopt(help = "The configuration file to load.", parse(from_os_str))]
    config: Option<PathBuf>,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            config: Some(default_config_path()),
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

    let args = Args::from_args();
    let conf_location = args.config.unwrap_or(default_config_path());

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
