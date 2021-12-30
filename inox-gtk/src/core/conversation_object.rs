use glib::subclass::prelude::*;
use inox_core::models::Conversation;
use once_cell::unsync::OnceCell;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use notmuch;

mod imp {
    use glib::subclass::prelude::*;
    use glib::Value;
    use glib::{ParamFlags, ParamSpec, ToValue};
    use inox_core::models::Conversation;
    use once_cell::sync::Lazy;
    use once_cell::unsync::OnceCell;

    #[derive(Clone, Debug)]
    pub struct ConversationObject {
        pub data: OnceCell<Conversation>,
    }

    impl Default for ConversationObject {
        fn default() -> Self {
            ConversationObject {
                data: OnceCell::new(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConversationObject {
        const NAME: &'static str = "InoxConversationObject";

        type Type = super::ConversationObject;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for ConversationObject {}
}

glib::wrapper! {
    pub struct ConversationObject(ObjectSubclass<imp::ConversationObject>);
}

// TODO: get from settings
const TAG_UNREAD: &str = "unread";
const TAG_ATTACHMENT: &str = "attachment";

impl Deref for ConversationObject {
    type Target = Conversation;

    fn deref(&self) -> &Self::Target {
        let imp = imp::ConversationObject::from_instance(self);
        imp.data.get().expect("ConversationObject object not set")
    }
}

impl ConversationObject {
    pub fn new(conversation: &Conversation) -> Self {
        let this = glib::Object::new(&[]).unwrap();
        let imp = imp::ConversationObject::from_instance(&this);
        imp.data
            .set(conversation.clone())
            .expect("ConversationObject object already set");
        this
    }

    pub fn data(&self) -> &Conversation {
        let imp = imp::ConversationObject::from_instance(self);
        imp.data.get().expect("ConversationObject object not set")
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.data().tags.contains(&tag.to_string())
    }

    pub fn is_unread(&self) -> bool {
        self.has_tag(TAG_UNREAD)
    }

    pub fn has_attachment(&self) -> bool {
        self.has_tag(TAG_ATTACHMENT)
    }
}
