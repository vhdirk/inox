use glib::subclass::prelude::*;
use glib::GBoxed;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use once_cell::unsync::OnceCell;

use notmuch;

use super::message::Message;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ThreadCache {
    is_unread: bool,
    has_attachment: bool,
    tags: Vec<String>,
}

impl Default for ThreadCache {
    fn default() -> Self {
        Self {
            is_unread: true,
            has_attachment: false,
            tags: vec![],
        }
    }
}


mod imp {
    use glib::subclass::prelude::*;
    use serde::{Deserialize, Serialize};
    use once_cell::unsync::OnceCell;
    use super::ThreadCache;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Thread {
        #[serde(skip)]
        thread: OnceCell<notmuch::Thread>,
        cache: ThreadCache,
    }

    impl Default for Thread {
        fn default() -> Self {
            Thread {
                thread: OnceCell::new(),
                cache: ThreadCache::default(),
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
        let t = glib::Object::new(&[]).unwrap();
        let imp = Self::from_instance(&t);
        imp.thread.set(thread);
        t
    }

    pub fn tags(&self) -> &Vec<String> {
        if let Some(thread) = &self.thread {
            let mut cache = self.cache.clone();
            cache.tags = thread.tags().collect()
        }
        &self.cache.tags
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(&tag.to_string())
    }

    pub fn is_unread(&self) -> bool {
        match &self.thread {
            Some(thread) => self.has_tag(TAG_UNREAD),
            None => self.cache.is_unread,
        }
    }

    pub fn has_attachment(&self) -> bool {
        self.has_tag(TAG_ATTACHMENT)
    }

    pub fn id(&self) -> &str {
        self.thread.as_ref().unwrap().id()
    }

    pub fn total_messages(&self) -> i32 {
        self.thread.as_ref().unwrap().total_messages()
    }

    // pub fn total_files(&self) -> i32 {
    //     self.thread.as_ref().unwrap().total_files()
    // }

    pub fn toplevel_messages(
        &self,
    ) -> notmuch::Messages {
        self.thread.as_ref().unwrap().toplevel_messages()
    }

    pub fn matched_messages(&self) -> i32 {
        self.thread.as_ref().unwrap().matched_messages()
    }

    pub fn messages(&self) -> Vec<Message> {
        self.thread.as_ref().unwrap().messages()
        .map(Message::new)
        .collect()
    }

    pub fn subject(&self) -> Cow<'_, str> {
        self.thread.as_ref().unwrap().subject()
    }

    pub fn authors(&self) -> Vec<String> {
        self.thread.as_ref().unwrap().authors()
    }

    /// Get the date of the oldest message in 'thread' as a time_t value.
    pub fn oldest_date(&self) -> i64 {
        self.thread.as_ref().unwrap().oldest_date()
    }

    /// Get the date of the newest message in 'thread' as a time_t value.
    pub fn newest_date(&self) -> i64 {
        self.thread.as_ref().unwrap().newest_date()
    }
}

// impl Iterator for Threads {
//     type Item = Thread;

//     fn next(self: &mut Self) -> Option<Self::Item> {
//         self.ref_rent_all_mut(|t| {
//             Thread::new(t.db.clone(), |db| t.query.clone(), |db, query| t.threads.next().unwrap());
//             true
//         });
//         None
//         //Some(Self::Item::new(cthread))
//     }
// }
