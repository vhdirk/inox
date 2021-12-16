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

use super::conversation_list_imp as imp;


// Wrap imp::ConversationList into a usable gtk-rs object
glib::wrapper! {
    pub struct ConversationList(ObjectSubclass<imp::ConversationList>)
        @extends gtk::Widget;
}

// ConversationList implementation itself
impl ConversationList {
    pub fn new(sender: Sender<Action>) -> Self {
        let thread_list: Self = glib::Object::new(&[]).expect("Failed to create ConversationList");
        let imp = imp::ConversationList::from_instance(&thread_list);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on ConversationList");
        thread_list.set_vexpand(true);
        thread_list.set_hexpand(true);
        thread_list
    }

    pub fn set_threads(&self, threads: notmuch::Threads) {
        let imp = imp::ConversationList::from_instance(self);
        let model = imp::create_liststore();

        for thread in threads {
            model.append(&Thread::new(thread));
        }

        imp.selection_model.set_model(Some(&model));
    }
}
