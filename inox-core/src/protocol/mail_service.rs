use std::result::Result;
use jsonrpc_derive::rpc;
use jsonrpc_core::BoxFuture;
use crate::models::{Conversation, Message, Query};


#[rpc]
pub trait MailService {
	type Metadata;

    // TODO: currently only supports 1 store (notmuch)

    // subscribe to any events concerning this email. These could be:
    // - new results to an existing query
    // - TODO
    // #[rpc(subscription = "mail/message/didOpen", subscribe, name = "hello_subscribe", alias("hello_alias"))]
    // fn subscribe(&self, metadata: Self::Metadata, subscriber: Subscriber<String>, c: u32, d: Option<u64>);

    #[rpc(meta, name = "mail/query/count_messages")]
    fn query_count_messages(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<u32, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/query/search_messages")]
    fn query_search_messages(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/query/count_conversations")]
    fn query_count_conversations(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<u32, jsonrpc_core::Error>>;

    // TODO: how to handle pagination?
    #[rpc(meta, name = "mail/query/search_conversations")]
    fn query_search_conversations(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<Vec<Conversation>, jsonrpc_core::Error>>;



    #[rpc(meta, name = "mail/conversation/toplevel_messages")]
    fn conversation_toplevel_messages(&self, state: Self::Metadata, conversation_id: String) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/conversation/messages")]
    fn conversation_messages(&self, state: Self::Metadata, conversation_id: String) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>>;

    /// Adds two numbers and returns a result
    #[rpc(meta, name = "mail/conversation/get")]
    fn conversation_get(&self, state: Self::Metadata, conversation_id: String) -> BoxFuture<Result<Option<Conversation>, jsonrpc_core::Error>>;


    // subscribe to any events concerning this email. These could be:
    // - tags (unread, bin, etc)
    // - TODO
    // #[rpc(subscription = "mail/message/didOpen", subscribe, name = "hello_subscribe", alias("hello_alias"))]
    // fn subscribe(&self, metadata: Self::Metadata, subscriber: Subscriber<String>, c: u32, d: Option<u64>);

    #[rpc(meta, name = "mail/message/get")]
    fn message_get(&self, state: Self::Metadata, message_id: String) -> BoxFuture<Result<Option<Message>, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/message/body")]
    fn message_body(&self, state: Self::Metadata, message_id: String, html: bool) -> BoxFuture<Result<Option<String>, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/conversation/replies")]
    fn message_replies(&self, state: Self::Metadata, message_id: String) -> BoxFuture<Result<Vec<Message>, jsonrpc_core::Error>>;


  	// subscribe to any events concerning this conversation. These could be:
    // - new reply
	// - TODO
    // #[rpc(subscription = "mail/message/didOpen", subscribe, name = "hello_subscribe", alias("hello_alias"))]
    // fn subscribe(&self, metadata: Self::Metadata, subscriber: Subscriber<String>, c: u32, d: Option<u64>);



}
