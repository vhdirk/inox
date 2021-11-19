use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use crate::core::thread::Thread;
use crate::core::message::Message;
use notmuch;

use crate::core::Action;
use super::message_view_imp as imp;

// Wrap imp::MessageView into a usable gtk-rs object
glib::wrapper! {
    pub struct MessageView(ObjectSubclass<imp::MessageView>)
        @extends gtk::Widget;
}

// MessageView implementation itself
impl MessageView {
    pub fn new(message: &notmuch::Message, sender: Sender<Action>) -> Self {
        let view: Self = glib::Object::new(&[]).expect("Failed to create MessageView");
        let imp = imp::MessageView::from_instance(&view);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on MessageView");

        imp.message
            .set(message.clone())
            .expect("Failed to set message on MessageView");


        let message = Message::from_file(message).unwrap();

        imp.parsed_message
            .set(message)
            .expect("Failed to set parsed_message on MessageView");


        imp.update();

        view
    }
}
