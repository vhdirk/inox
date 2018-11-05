use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::clone::Clone;
use std::convert::From;
use std::ops::Deref;

use std::collections::BTreeMap;
use toml;
use serde;

use notmuch;

// TODO: get from settings
const TAG_UNREAD: &'static str = "unread";
const TAG_ATTACHMENT: &'static str = "attachment";


pub trait ThreadEx {
    fn is_unread(&self) -> bool;
    fn has_attachment(&self) -> bool;
}

impl<'o, Owner: notmuch::ThreadOwner + 'o> ThreadEx for notmuch::Thread<'o, Owner>{

    // Does this thread carry the unread tag
    fn is_unread(&self) -> bool{
        let tags:Vec<String> = self.tags().collect();
        tags.contains(&TAG_UNREAD.to_string())
    }

    fn has_attachment(&self) -> bool{
        let tags:Vec<String> = self.tags().collect();
        tags.contains(&TAG_ATTACHMENT.to_string())
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
