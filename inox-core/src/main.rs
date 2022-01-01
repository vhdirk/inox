#![feature(iter_advance_by)]

use crate::handlers::state_metadata::StateMetadata;
use jsonrpc_core::MetaIoHandler;
use std::fs::DirBuilder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use dirs;
use gmime;
use notmuch;
use log::*;
use pretty_env_logger;

use jsonrpc_ipc_server::{RequestContext, ServerBuilder};
use jsonrpc_pubsub::{PubSubHandler, Session};
use structopt::StructOpt;

pub mod handlers;
pub mod models;
pub mod protocol;
pub mod settings;
pub mod convert;
pub mod util;
pub mod mime;

use handlers::mail_handler::MailHandler;
use protocol::mail_service::*;

use settings::Settings;

/// Init Gtk and logger.
fn init() {
    use std::sync::Once;

    static START: Once = Once::new();

    START.call_once(|| {
        pretty_env_logger::init();

        // run initialization here
        gmime::functions::init();
    });
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "inox-gtk",
    about = "An email client with notmuch rust.",
    author = "Dirk Van Haerenborgh <vhdirk@gmail.com>",
    version = "0.0.1"
)]
struct CoreArgs {
    #[structopt(help = "The configuration file to load.", parse(from_os_str))]
    config: Option<PathBuf>,
}

impl Default for CoreArgs {
    fn default() -> Self {
        CoreArgs {
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
    debug!("Starting Inox Core");

    init();

    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("inox");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap())
        .unwrap();

    let args = CoreArgs::from_args();
    let conf_path = args.config.unwrap_or_else(default_config_path);

    debug!("Using config file {:?}", conf_path);

    // load the settings
    let settings = Arc::new(Settings::new(conf_path.as_path()));

    let metaio_handler = MetaIoHandler::default();

    let mut io = PubSubHandler::new(metaio_handler);
    let mail_handler = MailHandler::default();
    io.extend_with(mail_handler.to_delegate());

    let socket_settings = settings.clone();
    let socket_path = socket_settings
        .as_ref()
        .inox_config
        .socket_path
        .to_str()
        .unwrap();

    let server = ServerBuilder::new(io)
        .session_meta_extractor(move |context: &RequestContext| StateMetadata {
            session_: Some(Arc::new(Session::new(context.sender.clone()))),
            settings: Some(settings.clone()),
        })
        .start(socket_path)
        .expect("Server must start without issues");

    debug!("Server listening on {}", &socket_path);
    server.wait();
}
