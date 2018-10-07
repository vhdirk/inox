use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
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

rental! {
    pub mod rent_notmuch {
        use super::*;

        #[rental]
        pub struct Query {
            db: Arc<notmuch::Database>,
            query: Rc<notmuch::Query<'db>>
        }

        #[rental]
        pub struct Thread {
            db: Arc<notmuch::Database>,
            query: Rc<notmuch::Query<'db>>,
            inner: notmuch::Thread<'query, 'db>
        }

    }
}




// #[derive(Default, Debug)]
// struct ThreadCache{

// }


// impl From<notmuch::Thread> for Thread{
//     fn from(thread: notmuch::Thread) -> Self{
//         Thread{
//             inner: thread,
//             cache: Rc::new(ThreadCache::default())
//         }
//     }
// }

// impl Deref for Thread{
//     type Target = notmuch::Thread;
//     fn deref(&self) -> &notmuch::Thread{
//         &self.inner
//     }
// }


// impl Thread{

//     // Does this thread carry the unread tag
//     pub fn is_unread(&self) -> bool{
//         let tags:Vec<String> = self.inner.tags().collect();
//         tags.contains(&TAG_UNREAD.to_string())
//     }

//     pub fn has_attachment(&self) -> bool{
//         let tags:Vec<String> = self.inner.tags().collect();
//         tags.contains(&TAG_ATTACHMENT.to_string())
//     }




// }
