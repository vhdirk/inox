use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::convert::From;
use std::ops::Deref;

use std::collections::BTreeMap;
use toml;
use serde;

use notmuch;

// TODO: get from settings
const TAG_UNREAD: &'static str = "unread";
const TAG_ATTACHMENT: &'static str = "attachment";

// Tiny wrapper around a notmuch Thread that does some basic caching and centralizes some
// functionality
#[derive(Clone, Debug)]
pub struct Thread{
    inner: notmuch::Thread,
    cache: Rc<ThreadCache>
}

#[derive(Default, Debug)]
struct ThreadCache{

}


impl From<notmuch::Thread> for Thread{
    fn from(thread: notmuch::Thread) -> Self{
        Thread{
            inner: thread,
            cache: Rc::new(ThreadCache::default())
        }
    }
}

impl Deref for Thread{
    type Target = notmuch::Thread;
    fn deref(&self) -> &notmuch::Thread{
        &self.inner
    }
}


impl Thread{

    // Does this thread carry the unread tag
    pub fn is_unread(&self) -> bool{
        let tags:Vec<String> = self.inner.tags().collect();
        tags.contains(&TAG_UNREAD.to_string())
    }

    pub fn has_attachment(&self) -> bool{
        let tags:Vec<String> = self.inner.tags().collect();
        tags.contains(&TAG_ATTACHMENT.to_string())
    }




}
