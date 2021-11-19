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


#[derive(Clone, Debug, GBoxed)]
#[gboxed(type_name = "inox_Message")]
pub struct Message {
    nm_message: notmuch::Message,

    // gm_message: ,
}

impl Message {
    pub fn new(message: &notmuch::Message) -> Self {
        Self {
            nm_message: message.clone(),
        }
    }
}
