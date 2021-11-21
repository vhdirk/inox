use crate::core::Action;
use crate::widgets::MessageRowBase;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

#[derive(Debug)]
pub struct MessagesView {
    pub list_box: gtk::ListBox,
    pub rows: RefCell<Vec<MessageRowBase>>,
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
    }

    fn dispose(&self, _obj: &Self::Type) {
        let mut rows = self.rows.borrow_mut();
        for row in rows.iter() {
            dbg!("row {:?}", row);
            row.unparent();
        }
        rows.clear();
        self.list_box.unparent();
    }
}
impl WidgetImpl for MessagesView {}
