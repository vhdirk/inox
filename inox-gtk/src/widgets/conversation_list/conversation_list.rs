use crate::core::ConversationObject;
use inox_core::models::Conversation;
use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::SingleSelection;
use gtk::{Application, SignalListItemFactory};
use log::*;

use crate::core::Action;

use super::conversation_list_imp as imp;


// Wrap imp::ConversationList into a usable gtk-rs object
glib::wrapper! {
    pub struct ConversationList(ObjectSubclass<imp::ConversationList>)
        @extends gtk::Widget;
}

// ConversationList implementation itself
impl ConversationList {
    pub fn new(sender: Sender<Action>) -> Self {
        let list: Self = glib::Object::new(&[]).expect("Failed to create ConversationList");
        let imp = imp::ConversationList::from_instance(&list);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on ConversationList");
        list.set_vexpand(true);
        list.set_hexpand(true);
        list
    }

    pub fn set_conversations(&self, conversations: &Vec<Conversation>) {
        let imp = imp::ConversationList::from_instance(self);
        let model = imp::create_liststore();

        for conversation in conversations {
            model.append(&ConversationObject::new(conversation));
        }

        imp.selection_model.set_model(Some(&model));
    }
}
