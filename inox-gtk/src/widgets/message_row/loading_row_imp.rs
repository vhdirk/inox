
use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::unsync::OnceCell;

use super::MessageRowBaseImpl;

#[derive(Debug, Default)]
pub struct LoadingRow {
    pub sender: OnceCell<Sender<Action>>,
    pub is_expanded: bool,
}

#[glib::object_subclass]
impl ObjectSubclass for LoadingRow {
    const NAME: &'static str = "InoxLoadingRow";
    type Type = super::LoadingRow;
    type ParentType = super::MessageRowBase;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for LoadingRow {
    fn constructed(&self, obj: &Self::Type) {
        // get_style_context().add_class(LOADING_CLASS);

        let spinner = gtk::Spinner::new();
        spinner.set_height_request(16);
        spinner.set_width_request(16);
        spinner.show();
        spinner.start();
        spinner.set_parent(obj);

        self.parent_constructed(obj);
    }
}
impl WidgetImpl for LoadingRow {}
impl ListBoxRowImpl for LoadingRow {}
impl MessageRowBaseImpl for LoadingRow {}
