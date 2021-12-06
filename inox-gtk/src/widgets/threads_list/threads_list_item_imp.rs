use std::cell::RefCell;
use crate::core::Action;
use crate::core::Thread;
use glib::subclass::prelude::*;
use glib::{clone, Sender};
use gtk;
use gtk::builders::ImageBuilder;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::SignalListItemFactory;
use once_cell::unsync::OnceCell;
use log::*;

pub fn create_liststore() -> gio::ListStore {
    gio::ListStore::new(Thread::static_type())
}

#[derive(Debug)]
pub struct ThreadsListItem {
    pub thread: RefCell<Option<Thread>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ThreadsListItem {
    const NAME: &'static str = "InoxThreadsListItem";
    type Type = super::ThreadsListItem;
    type ParentType = gtk::Widget;

    fn new() -> Self {
        Self {
            thread: RefCell::new(None),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for ThreadsListItem {
    fn constructed(&self, obj: &Self::Type) {
        // Setup

        // imp.column_view.set_parent(&imp.window);
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
    }
}
impl WidgetImpl for ThreadsListItem {}

impl ThreadsListItem {

}
