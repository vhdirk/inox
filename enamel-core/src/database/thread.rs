use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::rc::Rc;
use std::iter::Iterator;

use glib::{glib_boxed_type, glib_boxed_derive_traits};
use glib::subclass::boxed::BoxedType;

use notmuch;

#[derive(Clone, Debug)]
pub struct RcThread(Rc<notmuch::Thread<'static, 'static>>);

impl BoxedType for RcThread {
    const NAME: &'static str = "enamel::RcThread";
    glib_boxed_type!();
}
glib_boxed_derive_traits!(RcThread);

impl Deref for RcThread{
    type Target = Rc<notmuch::Thread<'static, 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RcThread{
    fn deref_mut(&mut self) -> &mut Rc<notmuch::Thread<'static, 'static>> {
        &mut self.0
    }
}

#[derive(Clone, Debug)]
pub struct ArcThread(Arc<notmuch::Thread<'static, 'static>>);

impl BoxedType for ArcThread {
    const NAME: &'static str = "enamel::ArcThread";
    glib_boxed_type!();
}
glib_boxed_derive_traits!(ArcThread);

impl Deref for ArcThread{
    type Target = Arc<notmuch::Thread<'static, 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ArcThread{
    fn deref_mut(&mut self) -> &mut Arc<notmuch::Thread<'static, 'static>> {
        &mut self.0
    }
}

// easy shorthand
pub type Thread = RcThread;



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
