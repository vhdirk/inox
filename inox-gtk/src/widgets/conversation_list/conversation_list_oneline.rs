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
// use crate::widgets::conversation_list_cell_renderer::CellRendererThread;
use crate::core::Thread;

use super::conversation_list_imp as imp;

const COLUMN_ID: u8 = 0;
const COLUMN_THREAD: u8 = 1;
const COLUMN_AUTHORS: u8 = 2;

// Wrap imp::ConversationList into a usable gtk-rs object
glib::wrapper! {
    pub struct ConversationList(ObjectSubclass<imp::ConversationList>)
        @extends gtk::Widget;
}

// ConversationList implementation itself
impl ConversationList {
    pub fn new(sender: Sender<Action>) -> Self {
        let conversation_list: Self = glib::Object::new(&[]).expect("Failed to create ConversationList");
        let imp = imp::ConversationList::from_instance(&conversation_list);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on ConversationList");
        conversation_list.set_vexpand(true);
        conversation_list.set_vexpand_set(true);

        conversation_list
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
