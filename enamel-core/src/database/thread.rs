use std::rc::Rc;
use std::iter::Iterator;

use notmuch;

// TODO: get from settings
const TAG_UNREAD: &str = "unread";
const TAG_ATTACHMENT: &str = "attachment";


pub trait ThreadExtra<'d, 'q>
where
    'd: 'q,
    Self: Sized
{
    fn has_tag(&self, tag: &str) -> bool;
    fn is_unread(&self) -> bool;
    fn has_attachment(&self) -> bool;

    // {
    //     let tags:Vec<String> = thread.tags().collect();
    //     tags.contains(&TAG_UNREAD.to_string())
    // }
    // fn has_attachment<'s, S>(thread: S) -> bool
    // where
    //     S: Into<Supercow<'s, notmuch::Thread<'o, O>>>,
    // {
    //     let tags:Vec<String> = notmuch::ThreadExt::tags(thread.into()).collect();
    //     tags.contains(&TAG_ATTACHMENT.to_string())
    // }  
}

impl<'d, 'q> ThreadExtra<'d, 'q> for notmuch::Thread<'d, 'q> where 'd: 'q {

    fn has_tag(&self, tag: &str) -> bool
    {
        let tags:Vec<String> = self.tags().collect();
        tags.contains(&tag.to_string())
    }

    fn is_unread(&self) -> bool
    {
        self.has_tag(TAG_UNREAD)
    }

    fn has_attachment(&self) -> bool
    {
        self.has_tag(TAG_ATTACHMENT)
    }
}

impl<'d, 'q> ThreadExtra<'d, 'q> for Rc<notmuch::Thread<'d, 'q>> where 'd: 'q {

    fn has_tag(&self, tag: &str) -> bool
    {
        let tags:Vec<String> = self.tags().collect();
        tags.contains(&tag.to_string())
    }

    fn is_unread(&self) -> bool
    {
        self.has_tag(TAG_UNREAD)
    }

    fn has_attachment(&self) -> bool
    {
        self.has_tag(TAG_ATTACHMENT)
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
