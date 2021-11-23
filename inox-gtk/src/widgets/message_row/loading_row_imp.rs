
use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::unsync::OnceCell;

use super::BaseRowImpl;

#[derive(Debug, Default)]
pub struct LoadingRow {
    pub sender: OnceCell<Sender<Action>>,
    pub spinner: gtk::Spinner,
}

#[glib::object_subclass]
impl ObjectSubclass for LoadingRow {
    const NAME: &'static str = "InoxLoadingRow";
    type Type = super::LoadingRow;
    type ParentType = super::BaseRow;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for LoadingRow {
    fn constructed(&self, obj: &Self::Type) {
        // get_style_context().add_class(LOADING_CLASS);

        self.spinner.set_height_request(16);
        self.spinner.set_width_request(16);
        self.spinner.show();
        self.spinner.start();
        self.spinner.set_parent(obj);

        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.spinner.unparent();
    }
}
impl WidgetImpl for LoadingRow {}
impl ListBoxRowImpl for LoadingRow {}
impl BaseRowImpl for LoadingRow {}
