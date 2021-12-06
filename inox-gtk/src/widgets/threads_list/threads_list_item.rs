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

use super::threads_list_item_imp as imp;


glib::wrapper! {
    pub struct ThreadsListItem(ObjectSubclass<imp::ThreadsListItem>)
        @extends gtk::Widget;
}

// ThreadsListItem implementation itself
impl ThreadsListItem {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ThreadsListItem")
    }

    pub fn set_thread(&self, thread: &Thread) {
        let imp = imp::ThreadsListItem::from_instance(self);
        imp.thread.replace(Some(thread.clone()));
    }
}
