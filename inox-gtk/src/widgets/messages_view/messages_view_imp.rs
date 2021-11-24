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
    // pub column_view: gtk::ColumnView,
    // pub model: gio::ListStore,
    // pub filter: gtk::TreeModelFilter,
    // idle_handle: RefCell<Option<glib::SourceId>>,
    // thread_list: RefCell<Option<Threads>>,

    // num_threads: u32,
    // num_threads_loaded: u32
    pub sender: OnceCell<Sender<Action>>,
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
            sender: OnceCell::new(),
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

        self.list_box
            .connect_row_activated(clone!(@weak obj => move |list_box, row| {
                let this = MessagesView::from_instance(&obj);
                this.on_row_activated(list_box, row)
            }));
    }

    fn dispose(&self, _obj: &Self::Type) {
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
