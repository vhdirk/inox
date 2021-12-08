use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::SingleSelection;
use gtk::{Application, SignalListItemFactory};
use log::*;
use notmuch;

use crate::core::Action;
use crate::core::Thread;

use super::thread_list_item_imp as imp;


glib::wrapper! {
    pub struct ThreadListItem(ObjectSubclass<imp::ThreadListItem>)
        @extends gtk::Box, gtk::Widget;
}

// ThreadListItem implementation itself
impl ThreadListItem {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ThreadListItem")
    }

    pub fn set_thread(&self, thread: &Thread) {
        let imp = imp::ThreadListItem::from_instance(self);
        imp.thread.replace(Some(thread.clone()));
        imp.update();
    }
}
