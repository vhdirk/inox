use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::rc::Rc;
use std::borrow::Cow;
use std::iter::Iterator;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use glib::{glib_boxed_type, glib_boxed_derive_traits};
use glib::subclass::boxed::BoxedType;

use notmuch;



#[derive(Clone, Debug, Serialize, Deserialize)]
struct MessageCache {
    is_unread: bool,
    has_attachment: bool,
    tags: Vec<String>
}

impl Default for MessageCache {

    fn default() -> Self {
        Self{
            is_unread: true,
            has_attachment: false,
            tags: vec![]
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message{

    #[serde(skip)]
    message: Option<Arc<notmuch::Message<'static, notmuch::Thread<'static, 'static>>>>,

    cache: MessageCache
}

impl BoxedType for Message {
    const NAME: &'static str = "inox_Message";
    glib_boxed_type!();
}
glib_boxed_derive_traits!(Message);


impl Message{
    pub fn new(message: notmuch::Message<'static, notmuch::Thread<'static, 'static>>) -> Self{
        Self{
            message: Some(Arc::new(message)),
            cache: MessageCache::default()
        }
    }

    fn safe_id(&self) -> String {
        let id = glib::markup_escape_text(&self.message.as_ref().unwrap().id());
        id.replace(",", "_")
    }


    pub fn id(self: &Self) -> Cow<'_, str> {
        self.message.as_ref().unwrap().id()
    }

    pub fn thread_id(self: &Self) -> Cow<'_, str> {
        self.message.as_ref().unwrap().thread_id()
    }

//     pub fn replies(self: &Self) -> Messages<'o, O> {
//         Messages::<'o, O>::from_ptr(
//             unsafe { ffi::notmuch_message_get_replies(self.ptr) },
//             // will never panic since the borrow is released immediately
//             ScopedPhantomcow::<'o, O>::share(&mut *(self.marker.borrow_mut()))
//         )
//     }

//     #[cfg(feature = "v0_26")]
//     pub fn count_files(self: &Self) -> i32 {
//         unsafe { ffi::notmuch_message_count_files(self.ptr) }
//     }

//     pub fn filenames(self: &Self) -> Filenames<Self> {
//         <Self as MessageExt<'o, O>>::filenames(self)
//     }

    pub fn filename(self: &Self) -> PathBuf {
        self.message.as_ref().unwrap().filename()
    }

//     pub fn date(&self) -> i64 {
//         unsafe { ffi::notmuch_message_get_date(self.ptr) as i64 }
//     }

//     pub fn header(&self, name: &str) -> Result<Option<Cow<'_, str>>> {
//         let name = CString::new(name).unwrap();
//         let ret = unsafe { ffi::notmuch_message_get_header(self.ptr, name.as_ptr()) };
//         if ret.is_null() {
//             Err(Error::UnspecifiedError)
//         } else {
//             let ret_str = ret.to_string_lossy();
//             if ret_str.is_empty() {
//                 Ok(None)
//             } else{
//                 Ok(Some(ret_str))
//             }
//         }
//     }

//     pub fn tags(&self) -> Tags<Self> {
//         <Self as MessageExt<'o, O>>::tags(self)
//     }

//     pub fn add_tag(self: &Self, tag: &str) -> Result<()> {
//         let tag = CString::new(tag).unwrap();
//         unsafe { ffi::notmuch_message_add_tag(self.ptr, tag.as_ptr()) }.as_result()
//     }

//     pub fn remove_tag(self: &Self, tag: &str) -> Result<()> {
//         let tag = CString::new(tag).unwrap();
//         unsafe { ffi::notmuch_message_remove_tag(self.ptr, tag.as_ptr()) }.as_result()
//     }

//     pub fn remove_all_tags(self: &Self) -> Result<()> {
//         unsafe { ffi::notmuch_message_remove_all_tags(self.ptr) }.as_result()
//     }

//     pub fn tags_to_maildir_flags(self: &Self) -> Result<()> {
//         unsafe { ffi::notmuch_message_tags_to_maildir_flags(self.ptr) }.as_result()
//     }

//     pub fn maildir_flags_to_tags(self: &Self) -> Result<()> {
//         unsafe { ffi::notmuch_message_maildir_flags_to_tags(self.ptr) }.as_result()
//     }

//     pub fn reindex<'d>(self: &Self, indexopts: IndexOpts<'d>) -> Result<()> {
//         unsafe { ffi::notmuch_message_reindex(self.ptr, indexopts.ptr) }.as_result()
//     }

//     pub fn freeze(self: &Self) -> Result<()> {
//         unsafe { ffi::notmuch_message_freeze(self.ptr) }.as_result()
//     }

//     pub fn thaw(self: &Self) -> Result<()> {
//         unsafe { ffi::notmuch_message_thaw(self.ptr) }.as_result()
//     }

//     pub fn properties<'m>(&'m self, key: &str, exact: bool) -> MessageProperties<'m, 'o, O> {
//         <Self as MessageExt<'o, O>>::properties(self, key, exact)
//     }

//     pub fn remove_all_properties(&self, key: Option<&str>) -> Result<()>
//     {
//         match key {
//             Some(k) => {
//                 let key_str = CString::new(k).unwrap();
//                 unsafe {
//                     ffi::notmuch_message_remove_all_properties(self.ptr, key_str.as_ptr())
//                 }.as_result()
//             },
//             None => {
//                 let p = ptr::null();
//                 unsafe {
//                     ffi::notmuch_message_remove_all_properties(self.ptr, p)
//                 }.as_result()
//             }
//         }
//     }

//     pub fn remove_all_properties_with_prefix(&self, prefix: Option<&str>) -> Result<()>
//     {
//         match prefix {
//             Some(k) => {
//                 let key_str = CString::new(k).unwrap();
//                 unsafe {
//                     ffi::notmuch_message_remove_all_properties_with_prefix(self.ptr, key_str.as_ptr())
//                 }.as_result()
//             },
//             None => {
//                 let p = ptr::null();
//                 unsafe {
//                     ffi::notmuch_message_remove_all_properties_with_prefix(self.ptr, p)
//                 }.as_result()
//             }
//         }
//     }


//     pub fn count_properties(&self, key: &str) -> Result<u32>
//     {
//         let key_str = CString::new(key).unwrap();
//         let mut cnt = 0;
//         unsafe {
//             ffi::notmuch_message_count_properties(self.ptr, key_str.as_ptr(), &mut cnt)
//         }.as_result()?;

//         Ok(cnt)
//     }

//     pub fn property(&self, key: &str) -> Result<Cow<'_, str>>
//     {
//         let key_str = CString::new(key).unwrap();
//         let mut prop = ptr::null();
//         unsafe {
//             ffi::notmuch_message_get_property(self.ptr, key_str.as_ptr(), &mut prop)
//         }.as_result()?;

//         if prop.is_null() {
//             Err(Error::UnspecifiedError)
//         } else {
//             // TODO: the unwrap here is not good
//             Ok(prop.to_string_lossy())
//         }
//     }

//     pub fn add_property(&self, key: &str, value: &str) -> Result<()>
//     {
//         let key_str = CString::new(key).unwrap();
//         let value_str = CString::new(value).unwrap();
//         unsafe {
//             ffi::notmuch_message_add_property(self.ptr, key_str.as_ptr(), value_str.as_ptr())
//         }.as_result()
//     }

//     pub fn remove_property(&self, key: &str, value: &str) -> Result<()>
//     {
//         let key_str = CString::new(key).unwrap();
//         let value_str = CString::new(value).unwrap();
//         unsafe {
//             ffi::notmuch_message_remove_property(self.ptr, key_str.as_ptr(), value_str.as_ptr())
//         }.as_result()
//     }

}
//   InternetAddressList * Message::to () {
//     if (missing_content) {
//       ustring s;

//       Db db (Db::DATABASE_READ_ONLY);
//       db.on_message (mid, [&](notmuch_message_t * msg)
//         {
//           /* read header field */
//           const char *c;

//           c = notmuch_message_get_header (msg, "To");

//           if (c != NULL) s = ustring (c);
//           else s = "";
//         });

//       LOG (debug) << "message: cached value: " << s;
//       if (s.empty ()) {
//         return internet_address_list_new ();
//       } else {
//         return internet_address_list_parse (NULL, s.c_str());
//       }
//     } else {
//       return g_mime_message_get_addresses (message, GMIME_ADDRESS_TYPE_TO);
//     }
//   }

