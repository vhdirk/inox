use std::sync::Arc;
use async_std::path::PathBuf;


use jsonrpc_pubsub::Session;
use jsonrpc_pubsub::PubSubMetadata;
use jsonrpc_core::Metadata;

use crate::settings::Settings;

#[derive(Debug, Clone, Default)]
pub struct StateMetadata {
    pub session_: Option<Arc<Session>>,
    pub settings: Option<Arc<Settings>>,
}

impl StateMetadata {

    pub async fn open_database(&self, mode: notmuch::DatabaseMode) -> Result<notmuch::Database, notmuch::Error> {
        let db_path = PathBuf::from(
            self.settings.as_ref().unwrap()
                .notmuch_config
                .database
                .path
                .clone(),
        );
        notmuch::Database::open(&db_path, mode)
    }

}

impl Metadata for StateMetadata {}

impl PubSubMetadata for StateMetadata {
	fn session(&self) -> Option<Arc<Session>> {
        return self.session_.clone();
    }

}