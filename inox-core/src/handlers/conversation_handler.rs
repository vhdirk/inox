use std::rc::Rc;
use std::result::Result;
use std::sync::Arc;
use jsonrpc_core::BoxFuture;

use crate::handlers::state_metadata::StateMetadata;
use crate::protocol::ConversationService;
use crate::settings::Settings;


pub struct ConversationHandler {
    settings: Arc<Settings>,
}

impl ConversationHandler {
    pub fn new(settings: Arc<Settings>) -> Self {
        Self { settings }
    }
}

impl ConversationService for ConversationHandler {
    type Metadata = StateMetadata;

    fn get(&self, state: Self::Metadata, id: String) -> BoxFuture<Result<u64, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(0)})
    }
}
