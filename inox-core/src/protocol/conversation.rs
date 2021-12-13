use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Conversation {
	/// Adds two numbers and returns a result
	#[rpc(name="get")]
	fn get(&self, id: String) -> Result<u64>;
}