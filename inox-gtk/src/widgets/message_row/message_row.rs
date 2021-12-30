use inox_core::models::Message;
use crate::core::Action;
use glib::Sender;

use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};

use crate::widgets::MessageView;
use crate::widgets::{ExpanderRow};
use super::message_row_imp as imp;

glib::wrapper! {
    pub struct MessageRow(ObjectSubclass<imp::MessageRow>)
    @extends ExpanderRow, gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MessageRow {
    pub fn new(message: &Message, sender: Sender<Action>) -> Self {
        let row: Self = glib::Object::new(&[]).expect("Failed to create MessageRow");

        let imp = imp::MessageRow::from_instance(&row);

        imp.sender
            .set(sender.clone())
            .expect("Failed to set sender on MessageRow");

        let view = MessageView::new(message, sender);
        imp.set_view(&view);

        row
    }
}
