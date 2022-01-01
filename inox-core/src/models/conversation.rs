use chrono::Utc;
use chrono::DateTime;
use crate::models::contact::Contact;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Conversation {
    pub id: String,
    pub total_messages: u32,
    pub matched_messages: u32,
    pub tags: Vec<String>,

    pub subject: String,
    pub authors: Vec<String>,

    pub oldest_date: DateTime<Utc>,
    pub newest_date: DateTime<Utc>,

    // TODO
    pub preview: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuerySearchConversations {
    pub conversations: Vec<Conversation>,
    pub total: u32
}
