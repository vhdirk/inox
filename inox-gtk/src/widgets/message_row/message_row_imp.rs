use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::unsync::OnceCell;
use crate::widgets::MessageView;

#[derive(Debug)]
pub struct MessageRow {
    pub sender: OnceCell<Sender<Action>>,
    pub message: OnceCell<notmuch::Message>,
    pub is_expanded: bool,
    pub view: OnceCell<MessageView>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageRow {
    const NAME: &'static str = "InoxMessageRow";
    type Type = super::MessageRow;
    type ParentType = super::MessageRowBase;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn new() -> Self {
        Self {
            sender: OnceCell::new(),
            message: OnceCell::new(),
            is_expanded: false,
            view: OnceCell::new(),
        }
    }
}

impl ObjectImpl for MessageRow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(view) = self.view.get() {
            view.unparent();
        }
    }
}
impl WidgetImpl for MessageRow {}
impl ListBoxRowImpl for MessageRow {}
impl super::MessageRowBaseImpl for MessageRow {}
