use crate::core::Action;
use crate::widgets::message_row::BaseRowExt;
use crate::widgets::BaseRow;
use glib::subclass::prelude::*;
use glib::{clone, Sender};
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

#[derive(Debug)]
pub struct MessagesView {
    pub list_box: gtk::ListBox,
    pub rows: RefCell<Vec<BaseRow>>,
    pub thread: OnceCell<notmuch::Thread>,
    pub sender: OnceCell<Sender<Action>>,

    pub row_activated_handler_id: RefCell<Option<glib::SignalHandlerId>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessagesView {
    const NAME: &'static str = "InoxMessagesView";
    type Type = super::MessagesView;
    type ParentType = gtk::Widget;

    fn new() -> Self {
        Self {
            list_box: gtk::ListBox::new(),
            rows: RefCell::new(vec![]),
            thread: OnceCell::new(),
            sender: OnceCell::new(),
            row_activated_handler_id: RefCell::new(None),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for MessagesView {
    fn constructed(&self, obj: &Self::Type) {
        self.list_box.set_parent(obj);
        self.parent_constructed(obj);

        self.row_activated_handler_id.replace(Some(self.list_box
            .connect_row_activated(clone!(@weak obj => move |list_box, row| {
                let this = MessagesView::from_instance(&obj);
                this.on_row_activated(list_box, row)
            }))));
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(id) = self.row_activated_handler_id.borrow_mut().take() {
            self.list_box.disconnect(id);
        }

        let mut rows = self.rows.borrow_mut();
        for row in rows.iter() {
            row.unparent();
        }
        rows.clear();
        self.list_box.unparent();
    }
}
impl WidgetImpl for MessagesView {}

impl MessagesView {
    pub fn on_row_activated(&self, list_box: &gtk::ListBox, row: &gtk::ListBoxRow) {
        let baserow = row.clone().downcast::<BaseRow>();
        if let Ok(row) = baserow {
            // Allow non-last rows to be expanded/collapsed, but also let
            // the last row to be expanded since appended sent emails will
            // be appended last. Finally, don't let rows with active
            // composers be collapsed.
            if row.property::<bool>("expanded") {
                if list_box.row_at_index(row.index() + 1).is_some() {
                    row.collapse();
                }
            } else {
                row.expand();
            }
        }
    }
}
