use chrono::Utc;
use chrono::DateTime;
use crate::models::contact::Contact;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    // pub recipients: Vec<Contact>,
    pub tags: Vec<String>,

    pub from_contacts: Vec<Contact>,
    pub to_contacts: Vec<Contact>,
    pub cc_contacts: Vec<Contact>,
    pub bcc_contacts: Vec<Contact>,
    pub reply_to_contacts: Vec<Contact>,

    pub date: DateTime<Utc>,
    pub subject: Option<String>,
    //pub preview: Option<String>,
}

pub struct MessageBody {
    pub body: Option<String>,

    // TODO: attachments
}