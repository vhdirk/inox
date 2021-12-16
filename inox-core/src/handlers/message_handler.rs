use std::result::Result;
use std::sync::Arc;
use async_std::path::PathBuf;
use jsonrpc_core::BoxFuture;
use crate::handlers::state_metadata::StateMetadata;
use crate::protocol::MessageService;
use crate::settings::Settings;
use crate::models::message::Message;


pub struct MessageHandler {
    settings: Arc<Settings>,
}

impl MessageHandler {
    pub fn new(settings: Arc<Settings>) -> Self {
        Self { settings }
    }

}

impl MessageService for MessageHandler {
    type Metadata = StateMetadata;

    fn get(&self, state: Self::Metadata, message_id: String) -> BoxFuture<Result<Option<Message>, jsonrpc_core::Error>> {
        Box::pin(async move {
            let db = state.open_database(notmuch::DatabaseMode::ReadOnly).await;

            if db.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }

            let msg = db.unwrap().find_message(&message_id);
            if msg.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            // msg.map(|msg| {
            //     msg.
            // });

            Ok(None)
        })

    }

    fn body(&self, state: Self::Metadata, message_id: String, html: bool) -> BoxFuture<Result<String, jsonrpc_core::Error>> {
        Box::pin(async move {
            Ok("".to_string())
        })
    }
}
