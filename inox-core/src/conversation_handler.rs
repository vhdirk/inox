use jsonrpc_core::Result;
use super::protocol::conversation::Conversation;

pub struct ConversationHandler {
}

impl Conversation for ConversationHandler {

	fn get(&self, id: String) -> Result<u64> {
        Ok(0)
    }

}