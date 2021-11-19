use crate::core::Action;
use glib::Sender;

use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};

use crate::widgets::MessageView;

use super::{MessageRowBase};
use super::message_row_imp as imp;

glib::wrapper! {
    pub struct MessageRow(ObjectSubclass<imp::MessageRow>)
    @extends MessageRowBase, gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MessageRow {
    pub fn new(message: &notmuch::Message, sender: Sender<Action>) -> Self {
        let row: Self = glib::Object::new(&[]).expect("Failed to create MessageRow");
        let imp = imp::MessageRow::from_instance(&row);

        imp.sender
            .set(sender.clone())
            .expect("Failed to set sender on MessageRow");
        row.set_vexpand(true);
        row.set_vexpand_set(true);

        imp.message
            .set(message.clone())
            .expect("Failed to set message on MessageRow");

        let view = MessageView::new(message, sender);
        row.set_child(Some(&view));

        imp.view
            .set(view)
            .expect("Failed to set view on MessageRow");
        row
    }
}
