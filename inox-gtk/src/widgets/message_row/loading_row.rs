use crate::core::Action;
use glib::Sender;

use glib::{self, subclass::prelude::*};
use gtk::{self, prelude::*};

use super::loading_row_imp as imp;
use super::base_row::BaseRow;

glib::wrapper! {
    pub struct LoadingRow(ObjectSubclass<imp::LoadingRow>)
    @extends BaseRow, gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl LoadingRow {
    pub fn new(sender: Sender<Action>) -> Self {
        let row: Self = glib::Object::new(&[]).expect("Failed to create LoadingRow");
        let imp = imp::LoadingRow::from_instance(&row);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on LoadingRow");
        row.set_vexpand(true);
        row.set_vexpand_set(true);

        row
    }
}
