use std::result::Result;
use jsonrpc_derive::rpc;
use jsonrpc_core::BoxFuture;
use crate::models::query::Query;

#[rpc]
pub trait QueryService {
	type Metadata;

    // TODO: currently only supports 1 store (notmuch)

    // subscribe to any events concerning this email. These could be:
    // - new results to an existing query
    // - TODO
    // #[rpc(subscription = "mail/message/didOpen", subscribe, name = "hello_subscribe", alias("hello_alias"))]
    // fn subscribe(&self, metadata: Self::Metadata, subscriber: Subscriber<String>, c: u32, d: Option<u64>);

    #[rpc(meta, name = "mail/query/count_messages")]
    fn count_messages(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<u32, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/query/messages")]
    fn messages(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<u32, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/query/count_threads")]
    fn count_threads(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<u32, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/query/threads")]
    fn threads(&self, state: Self::Metadata, query: Query) -> BoxFuture<Result<u32, jsonrpc_core::Error>>;

}
