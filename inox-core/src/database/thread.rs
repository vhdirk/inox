use glib::subclass::prelude::*;
use glib::GBoxed;
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
    use once_cell::sync::Lazy;
    use once_cell::unsync::OnceCell;

    #[derive(Clone, Debug)]
    pub struct Thread {
        pub data: OnceCell<notmuch::Thread>,
    }

    impl Default for Thread {
        fn default() -> Self {
            Thread {
                data: OnceCell::new(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Thread {
        const NAME: &'static str = "InoxThread";

        type Type = super::Thread;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for Thread {}
}

glib::wrapper! {
    pub struct Thread(ObjectSubclass<imp::Thread>);
}

// TODO: get from settings
const TAG_UNREAD: &str = "unread";
const TAG_ATTACHMENT: &str = "attachment";

impl Thread {
    pub fn new(thread: notmuch::Thread) -> Self {
        let this = glib::Object::new(&[]).unwrap();
        let imp = imp::Thread::from_instance(&this);
        imp.data.set(thread).expect("Thread object already set");
        this
    }

    pub fn data(&self) -> &notmuch::Thread {
        let imp = imp::Thread::from_instance(self);
        imp.data.get().expect("Thread object not set")
    }
        pub fn tags(&self) -> Vec<String> {
        self.data().tags().collect()
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(&tag.to_string())
    }

    pub fn is_unread(&self) -> bool {
        self.has_tag(TAG_UNREAD)
    }

    pub fn has_attachment(&self) -> bool {
        self.has_tag(TAG_ATTACHMENT)
    }

}
