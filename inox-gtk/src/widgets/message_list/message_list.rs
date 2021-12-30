use inox_core::models::Message;
use gio::prelude::*;
use gtk::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;

use notmuch;

use crate::core::Action;


use super::message_list_imp as imp;

// Wrap imp::MessageList into a usable gtk-rs object
glib::wrapper! {
    pub struct MessageList(ObjectSubclass<imp::MessageList>)
        @extends gtk::Widget;
}

// MessageList implementation itself
impl MessageList {
    pub fn new(messages: &Vec<Message>, sender: Sender<Action>) -> Self {
        let view: Self = glib::Object::new(&[]).expect("Failed to create MessageList");
        let imp = imp::MessageList::from_instance(&view);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on MessageList");
        imp.messages
            .set(messages.clone())
            .expect("Failed to set messages on MessageList");

        view.set_vexpand(true);
        view.set_hexpand(true);

        imp.init();

        view
    }
}
