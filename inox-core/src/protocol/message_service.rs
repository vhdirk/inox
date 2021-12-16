use std::result::Result;
use jsonrpc_derive::rpc;
use jsonrpc_core::BoxFuture;
use crate::models::message::Message;

#[rpc]
pub trait MessageService {
	type Metadata;



    // subscribe to any events concerning this email. These could be:
    // - tags (unread, bin, etc)
    // - TODO
    // #[rpc(subscription = "mail/message/didOpen", subscribe, name = "hello_subscribe", alias("hello_alias"))]
    // fn subscribe(&self, metadata: Self::Metadata, subscriber: Subscriber<String>, c: u32, d: Option<u64>);

    #[rpc(meta, name = "mail/message/get")]
    fn get(&self, state: Self::Metadata, message_id: String) -> BoxFuture<Result<Option<Message>, jsonrpc_core::Error>>;

    #[rpc(meta, name = "mail/message/body")]
    fn body(&self, state: Self::Metadata, message_id: String, html: bool) -> BoxFuture<Result<String, jsonrpc_core::Error>>;



}
