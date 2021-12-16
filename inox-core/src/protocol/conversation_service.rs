use jsonrpc_core::BoxFuture;
use jsonrpc_derive::rpc;

#[rpc]
pub trait ConversationService {
	type Metadata;

  	// subscribe to any events concerning this conversation. These could be:
    // - new reply
	// - TODO
    // #[rpc(subscription = "mail/message/didOpen", subscribe, name = "hello_subscribe", alias("hello_alias"))]
    // fn subscribe(&self, metadata: Self::Metadata, subscriber: Subscriber<String>, c: u32, d: Option<u64>);


	/// Adds two numbers and returns a result
	#[rpc(meta, name = "mail/conversation/get")]
	fn get(&self, state: Self::Metadata, id: String) -> BoxFuture<Result<u64, jsonrpc_core::Error>>;
}
