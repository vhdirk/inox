use crate::convert::message_helper::MessageHelper;
use crate::database::DatabaseExt;
use crate::handlers::state_metadata::StateMetadata;
use crate::models::query::Exclude;
use crate::models::QuerySearchConversations;
use crate::models::{self, Conversation, Message, Query, Sort};
use crate::protocol::MailService;
use crate::settings::Settings;
use async_std::path::PathBuf;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use jsonrpc_core::BoxFuture;
use log::*;
use std::result::Result;

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

impl From<notmuch::Message> for Message {
    fn from(msg: notmuch::Message) -> Message {
        let helper = MessageHelper::new(&msg).unwrap();

        Message {
            id: msg.id().to_string(),
            tags: msg.tags().collect(),

            from_contacts: helper.from_contacts(),
            to_contacts: helper.to_contacts(),
            cc_contacts: helper.cc_contacts(),
            bcc_contacts: helper.bcc_contacts(),
            reply_to_contacts: helper.reply_to_contacts(),

            date: helper.date(),
            subject: helper.subject()
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
            total_messages: thread.total_messages() as u32,
            matched_messages: thread.matched_messages() as u32,
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
            for tag in query.exclude_tags.iter() {
                q.add_tag_exclude(tag);
            }
            q.set_omit_excluded(query.exclude.clone().into());
            q
        })
    }
}

impl MailService for MailHandler {
    type Metadata = StateMetadata;

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

    // TODO: pagination?
    fn query_search_conversations(
        &self,
        state: Self::Metadata,
        query: Query,
    ) -> BoxFuture<Result<QuerySearchConversations, jsonrpc_core::Error>> {
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

            let q = q.unwrap();
            let count = q.count_messages();
            if count.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }

            let threads = q.search_threads();
            if threads.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }
            let mut conversations = vec![];

            let threads = threads.unwrap();

            for (i, thread) in threads.enumerate() {
                // skip messages until we reach offset
                if let Some(offset) = query.offset {
                    if i < (offset as usize) {
                        continue;
                    }
                }

                conversations.push(thread.into());

                // stop if we have enough results
                if let Some(limit) = query.limit {
                    let offset = query.offset.unwrap_or(0);
                    if i < ((offset + limit) as usize) {
                        break;
                    }
                }
            }

            Ok(QuerySearchConversations {
                conversations,
                total: count.unwrap(),
            })
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

    fn message_replies(
        &self,
        state: Self::Metadata,
        message_id: String,
    ) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(vec![]) })
    }

    fn conversation_toplevel_messages(
        &self,
        state: Self::Metadata,
        conversation_id: String,
    ) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(vec![]) })
    }

    fn conversation_messages(
        &self,
        state: Self::Metadata,
        conversation_id: String,
    ) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>> {
        // limit/offset needed?
        debug!("conversation_messages: {:?} {:?}", state, conversation_id);

        Box::pin(async move {
            let db = state.open_database(notmuch::DatabaseMode::ReadOnly).await;

            if db.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }

            let thread = db.unwrap().find_thread_by_id(&conversation_id);

            if thread.is_err() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }

            let thread = thread.unwrap();

            if thread.is_none() {
                // TODO
                return Err(jsonrpc_core::Error::internal_error());
            }

            let messages = thread.unwrap().messages();

            let mut msgs = vec![];

            for message in messages {
                msgs.push(message.into());
            }

            Ok(msgs)
        })
    }

    fn conversation_get(
        &self,
        state: Self::Metadata,
        conversation_id: String,
    ) -> BoxFuture<Result<Option<Conversation>, jsonrpc_core::Error>> {
        Box::pin(async move { Ok(None) })
    }
}
