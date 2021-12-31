use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::DateTime;
use crate::handlers::state_metadata::StateMetadata;
use crate::models::query::Exclude;
use crate::models::{self, Conversation, Message, Query, Sort};
use crate::protocol::MailService;
use crate::settings::Settings;
use crate::convert::message_helper::MessageHelper;
use async_std::path::PathBuf;
use jsonrpc_core::BoxFuture;
use std::result::Result;
use log::*;

#[derive(Default)]
pub struct MailHandler {}

impl Into<notmuch::Sort> for Sort {
    fn into(self) -> notmuch::Sort {
        match self {
            models::query::Sort::OldestFirst => notmuch::Sort::OldestFirst,
            models::query::Sort::NewestFirst => notmuch::Sort::NewestFirst,
            models::query::Sort::MessageID => notmuch::Sort::MessageID,
            models::query::Sort::Unsorted => notmuch::Sort::Unsorted,
        }
    }
}

impl Into<notmuch::Exclude> for Exclude {
    fn into(self) -> notmuch::Exclude {
        match self {
            models::query::Exclude::Flag => notmuch::Exclude::Flag,
            models::query::Exclude::True => notmuch::Exclude::True,
            models::query::Exclude::False => notmuch::Exclude::False,
            models::query::Exclude::All => notmuch::Exclude::All,
        }
    }
}

impl From<&notmuch::Message> for Message {
    fn from(msg: &notmuch::Message) -> Message {

        let helper = MessageHelper::new(msg);

        Message {
            id: msg.id().to_string(),
            tags: msg.tags().collect(),

            from_contacts: vec![],
            to_contacts: vec![],
            cc_contacts: vec![],
            bcc_contacts: vec![],
            reply_to_contacts: vec![],


            // recipients:
            // from_contacts:
            // to_contacts:
            // cc_contacts:
            // bcc_contacts:
            // reply_to_contacts:

            date: None, // date: msg.date(),
            subject: None, //msg.subject().to_string(),
        }
    }
}

impl From<notmuch::Thread> for Conversation {
    fn from(thread: notmuch::Thread) -> Conversation {
        Conversation {
            id: thread.id().to_string(),
            subject: thread.subject().to_string(),
            tags: thread.tags().collect(),
            authors: thread.authors(),
            oldest_date: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(thread.oldest_date(), 0),
                Utc,
            ),

            newest_date: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(thread.newest_date(), 0),
                Utc,
            ),
            preview: None,
            total_messages: thread.total_messages(),
            matched_messages: thread.matched_messages(),
        }
    }
}

impl MailHandler {
    pub fn convert_query(
        query: &Query,
        db: &notmuch::Database,
    ) -> Result<notmuch::Query, notmuch::Error> {
        let res = db.create_query(&query.query);
        res.map(move |q| {
            q.set_sort(query.sort.clone().into());
            for tag in query.tags_exclude.iter() {
                q.add_tag_exclude(tag);
            }
            q.set_omit_excluded(query.omit_excluded.clone().into());
            q
        })
    }
}

impl MailService for MailHandler {
    type Metadata = StateMetadata;

    fn query_count_messages(
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

            if dbquery.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            let count = dbquery.unwrap().count_messages();

            if count.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            Ok(count.unwrap())
        })
    }

    fn query_search_messages(
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

    fn query_count_conversations(
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

    // TODO: pagination?
    fn query_search_conversations(
        &self,
        state: Self::Metadata,
        query: Query,
    ) -> BoxFuture<Result<Vec<Conversation>, jsonrpc_core::Error>> {

        debug!("query_search_conversations: {:?} {:?}", state, query);

        Box::pin(async move {
            let db = state.open_database(notmuch::DatabaseMode::ReadOnly).await;

            if db.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }

            let q = MailHandler::convert_query(&query, &db.unwrap());

            if q.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            let threads = q.unwrap().search_threads();
            if threads.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            let mut conversations = vec![];
            for thread in threads.unwrap() {
                conversations.push(thread.into());
            }

            Ok(conversations)
        })
    }

    fn message_get(
        &self,
        state: Self::Metadata,
        message_id: String,
    ) -> BoxFuture<Result<Option<Message>, jsonrpc_core::Error>> {
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

    fn message_body(
        &self,
        state: Self::Metadata,
        message_id: String,
        html: bool,
    ) -> BoxFuture<Result<Option<String>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(None) })
    }

    fn message_replies(&self, state: Self::Metadata, message_id: String) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(vec![]) })
    }

    fn conversation_toplevel_messages(&self, state: Self::Metadata, conversation_id: String) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(vec![]) })
    }


    fn conversation_messages(&self, state: Self::Metadata, conversation_id: String) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(vec![]) })
    }

    fn conversation_get(
        &self,
        state: Self::Metadata,
        conversation_id: String,
    ) -> BoxFuture<Result<Option<Conversation>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(None) })
    }


}
