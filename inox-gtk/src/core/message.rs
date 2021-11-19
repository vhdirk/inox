use std::path::Path;
use glib::subclass::boxed::BoxedType;
use glib::GBoxed;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use notmuch;
use gmime::traits::ParserExt;


#[derive(Clone, Debug, GBoxed)]
#[gboxed(type_name = "inox_Message")]
pub struct Message {
    message: gmime::Message,
}

impl Deref for Message {
    type Target = gmime::Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl Message {
    pub fn from_file(message: &notmuch::Message) -> Result<Self, glib::Error> {

        // create a stream to read from the file descriptor
        // TODO: proper error handling
        let stream = gmime::StreamFile::open(&message.filename().to_string_lossy(), "r")?;

        // create a new parser object to parse the stream
        let parser = gmime::Parser::with_stream(&stream);

        // parse the message from the stream
        // TODO: what if empty?
        let message = parser.construct_message(None).unwrap();

        Ok(Self {
            message
        })
    }
}

