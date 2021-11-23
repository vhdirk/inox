use crate::core::Action;
use glib::Sender;

use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};

use crate::widgets::MessageView;

use super::{BaseRow};
use super::message_row_imp as imp;

glib::wrapper! {
    pub struct MessageRow(ObjectSubclass<imp::MessageRow>)
    @extends BaseRow, gtk::ListBoxRow, gtk::Widget,
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



    // pub fn expand()
    //         throws GLib.Error {
    //         this.is_expanded = true;
    //         update_row_expansion();
    //         if (this.view.message_body_state == NOT_STARTED) {
    //             yield this.view.load_body();
    //             email_loaded(this.view.email);
    //         }
    //     }

    pub fn collapse(&self) {
        let expanded = self.set_property("expanded", false);
        let pinned = self.set_property("pinned", false);

        self.update_row_expansion();
    }


    pub fn update_row_expansion(&self) {
        let expanded = self.property::<bool>("expanded");
        let pinned = self.property::<bool>("pinned");

        let imp = imp::MessageRow::from_instance(self);

        if (expanded || pinned) {
            imp.expand();
        } else {
            imp.collapse();
        }
    }
}
