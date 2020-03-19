use tarpc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub email: String,
    pub full_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub id: i32,
    pub sid: String,
    pub mime_type: String,
    pub cid: String,

    pub viewable: bool,
    pub preferred: bool,
    pub attachment: bool,

    pub encrypted: bool,
    pub signed: bool,  // 'signed' doesn't work

    pub crypto_id: i32,

    pub signature: ChunkSignature,
    pub encryption: ChunkEncryption,

    pub sibling: bool,
    pub used: bool,
    pub focusable: bool,

    pub content: String,
    pub filename: String,
    pub size: u32,
    pub human_size: String,

    pub thumbnail: String, // used by attachments

    pub children: Vec<Chunk>,
    pub siblings: Vec<Chunk>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkSignature {
    pub verified: bool,

    pub sign_strings: Vec<String>,
    pub all_errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkEncryption {
    pub decrypted: bool,
    pub enc_strings: Vec<String>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender: Address,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub bcc: Vec<Address>,
    pub reply_to: Vec<Address>,

    pub date_pretty: String,
    pub date_verbose: String,

    pub subject: String,
    pub tags: Vec<String>,
    pub tag_string: String,

    pub gravatar: String,

    pub missing_content: bool,
    pub patch: bool,
    pub different_subject: bool,

    pub level: i32,
    pub in_reply_to: String,

    pub preview: String,

    pub root: Chunk,

    pub mime_messages: Vec<Chunk>,
    pub attachments: Vec<Chunk>
}





#[tarpc::service]
pub trait Page {
    async fn allow_remote_images(name: String) -> ();

    async fn load(html_content: String,
                  css_content: String,
                  part_css: Option<String>,
                  allowed_uris: Vec<String>,
                  use_stdout: bool,
                  use_syslog: bool,
                  disable_log: bool,
                  log_level: String) -> ();

    async fn clear_messages() -> ();

    async fn add_message(messages: Message);
    async fn add_messages(messages: Vec<Message>);

    // async fn update_message_states( edit_mode: bool);

//         message_id: String,


//   message MessageState {
//     string mid = 1;

//     message Element {
//       enum Type {
//         Empty       = 0;
//         Address     = 1;
//         Part        = 2;
//         Attachment  = 3;
//         MimeMessage = 4;
//         Encryption  = 5;
//       }

//       Type   type = 1;
//       int32  id   = 2;
//       string sid  = 3;
//       bool   focusable = 4;
//     }

//     repeated Element elements = 5;
//     int32    level = 6;
//   }

//   repeated MessageState messages = 2;
//   bool edit_mode = 3;

//     )

}


