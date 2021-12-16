use crate::handlers::state_metadata::StateMetadata;
use jsonrpc_core::MetaIoHandler;
use jsonrpc_tcp_server::RequestContext;
use std::fs::DirBuilder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use dirs;
use gmime;
use log::*;
use pretty_env_logger;

use jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_pubsub::{PubSubHandler, Session, Subscriber, SubscriptionId};
use jsonrpc_tcp_server::ServerBuilder;
use structopt::clap::{App, Arg};
use structopt::StructOpt;

pub mod handlers;
pub mod models;
pub mod protocol;
pub mod settings;

use handlers::conversation_handler::ConversationHandler;
use protocol::conversation_service::*;

use handlers::message_handler::MessageHandler;
use protocol::message_service::*;

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
    init();

    let mut default_config = dirs::config_dir().unwrap();
    default_config.push("inox");

    DirBuilder::new()
        .recursive(true)
        .create(default_config.to_str().unwrap())
        .unwrap();

    let args = CoreArgs::from_args();
    let conf_location = args.config.unwrap_or(default_config_path());

    debug!("Using config file {:?}", conf_location);

    // load the settings
    let conf_path: PathBuf = PathBuf::from(conf_location);
    let settings = Arc::new(Settings::new(&conf_path.as_path()));

    let metaio_handler = MetaIoHandler::default();

    let mut io = PubSubHandler::new(metaio_handler);
    let conversation_handler = ConversationHandler::new(settings.clone());
    io.extend_with(conversation_handler.to_delegate());

    let message_handler = MessageHandler::new(settings.clone());
    io.extend_with(message_handler.to_delegate());

    let serve_address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        settings.as_ref().inox_config.port,
    );

    let server = ServerBuilder::new(io)
        .session_meta_extractor(move |context: &RequestContext| StateMetadata {
            session_: Some(Arc::new(Session::new(context.sender.clone()))),
            settings: Some(settings.clone()),
        })
        .start(&serve_address)
        .expect("Server must start with no issues");

    server.wait();
}
