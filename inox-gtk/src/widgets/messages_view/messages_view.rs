use gio::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;

use notmuch;

use crate::core::Action;
use crate::widgets::MessageRow;
use crate::widgets::BaseRow;

use super::messages_view_imp as imp;

// Wrap imp::MessagesView into a usable gtk-rs object
glib::wrapper! {
    pub struct MessagesView(ObjectSubclass<imp::MessagesView>)
        @extends gtk::Widget;
}

// MessagesView implementation itself
impl MessagesView {
    pub fn new(thread: &notmuch::Thread, sender: Sender<Action>) -> Self {
        let view: Self = glib::Object::new(&[]).expect("Failed to create MessagesView");
        let imp = imp::MessagesView::from_instance(&view);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on MessagesView");
        // view.set_vexpand(true);
        // view.set_vexpand_set(true);
        view.load_messages(thread);
        view
    }

    pub fn load_messages(&self, thread: &notmuch::Thread) {
        let messages = thread.messages();
        for message in messages {
            self.add_message(&message);
        }
    }

    pub fn add_message(&self, message: &notmuch::Message) {
        let imp = imp::MessagesView::from_instance(self);
        let message_row = MessageRow::new(message, imp.sender.get().unwrap().clone());
        imp.list_box.append(&message_row);
        imp.rows
            .borrow_mut()
            .push(message_row.upcast::<BaseRow>());
    }

    pub fn clear(&self) {
        // self.list_box.foreach(|child| {
        //     self.list_box.remove(&child);
        // });
    }
}
