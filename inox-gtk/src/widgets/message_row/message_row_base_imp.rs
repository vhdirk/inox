use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::unsync::OnceCell;

#[derive(Debug, Default)]
pub struct MessageRowBase {
    pub sender: OnceCell<Sender<Action>>,
    pub is_expanded: bool,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageRowBase {
    const NAME: &'static str = "InoxMessageRowBase";
    const ABSTRACT: bool = true;
    type Type = super::MessageRowBase;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for MessageRowBase {}
impl WidgetImpl for MessageRowBase {}
impl ListBoxRowImpl for MessageRowBase {}
