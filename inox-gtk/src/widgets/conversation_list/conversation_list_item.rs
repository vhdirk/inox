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

use super::conversation_list_item_imp as imp;


glib::wrapper! {
    pub struct ConversationListItem(ObjectSubclass<imp::ConversationListItem>)
        @extends gtk::Box, gtk::Widget;
}

// ConversationListItem implementation itself
impl ConversationListItem {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ConversationListItem")
    }

    pub fn set_thread(&self, thread: &Thread) {
        let imp = imp::ConversationListItem::from_instance(self);
        imp.thread.replace(Some(thread.clone()));
        imp.update();
    }
}
