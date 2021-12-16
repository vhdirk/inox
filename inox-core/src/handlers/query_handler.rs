use crate::handlers::state_metadata::StateMetadata;
use crate::models::{Conversation, Message, Query};
use crate::protocol::QueryService;
use crate::settings::Settings;
use async_std::path::PathBuf;
use jsonrpc_core::BoxFuture;
use std::result::Result;
use std::sync::Arc;

pub struct QueryHandler {}

impl QueryHandler {}

impl QueryService for QueryHandler {
    type Metadata = StateMetadata;

    fn count_messages(
        &self,
        state: Self::Metadata,
        query: Query,
    ) -> BoxFuture<Result<u32, jsonrpc_core::Error>> {
        Box::pin(async move {
            let db = state.open_database(notmuch::DatabaseMode::ReadOnly).await;

            if db.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }

            let dbquery = db.unwrap().create_query(&query.query);

            Ok(0)
        })
    }

    fn messages(
        &self,
        state: Self::Metadata,
        query: Query,
    ) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>> {
        Box::pin(async move {
            let db = state.open_database(notmuch::DatabaseMode::ReadOnly).await;

            if db.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            Ok(vec![])
        })
    }

    fn count_conversations(
        &self,
        state: Self::Metadata,
        query: Query,
    ) -> BoxFuture<Result<u32, jsonrpc_core::Error>> {
        Box::pin(async move {
            let db = state.open_database(notmuch::DatabaseMode::ReadOnly).await;

            if db.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            Ok(0)
        })
    }

    fn conversations(
        &self,
        state: Self::Metadata,
        query: Query,
    ) -> BoxFuture<Result<Vec<Conversation>, jsonrpc_core::Error>> {
        Box::pin(async move {
            let db = state.open_database(notmuch::DatabaseMode::ReadOnly).await;

            if db.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            Ok(vec![])
        })
    }
}
