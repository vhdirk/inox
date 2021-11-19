
use crate::core::Action;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::core::Thread;
use once_cell::unsync::OnceCell;
// use crate::widgets::thread_list_cell_renderer::CellRendererThread;

// pub fn append_text_column(tree: &gtk::TreeView, id: i32, title: &str) {
//     let column = gtk::TreeViewColumn::new();
//     let cell = CellRendererThread::new();
//     column.pack_start(&cell, false);
//     // Association of the view's column with the model's `id` column.
//     column.add_attribute(&cell, "thread", id);
//     column.set_title(&title);
//     tree.append_column(&column);
// }

pub fn create_liststore() -> gio::ListStore {
    gio::ListStore::new(Thread::static_type())
}

#[derive(Debug)]
pub struct ThreadsList {
    pub scrolled_window: gtk::ScrolledWindow,
    pub column_view: gtk::ColumnView,
    pub model: gio::ListStore,
    pub selection_model: gtk::SingleSelection,

    // pub filter: gtk::TreeModelFilter,
    // idle_handle: RefCell<Option<glib::SourceId>>,
    // thread_list: RefCell<Option<Threads>>,

    // num_threads: u32,
    // num_threads_loaded: u32
    pub sender: OnceCell<Sender<Action>>,
}

// impl Default for ThreadsList {
//     fn default() -> Self {

//     }
// }

#[glib::object_subclass]
impl ObjectSubclass for ThreadsList {
    const NAME: &'static str = "InoxThreadsList";
    type Type = super::ThreadsList;
    type ParentType = gtk::Widget;

    fn new() -> Self {
        let model = gio::ListStore::new(Thread::static_type());
        let selection_model = gtk::SingleSelection::new(Some(&model));
        let column_view = gtk::ColumnView::new(Some(&selection_model));
        let scrolled_window = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .vexpand_set(true)
            .child(&column_view)
            .build();

        scrolled_window.show();
        column_view.show();

        Self {
            scrolled_window,
            column_view,
            model,
            selection_model,
            sender: OnceCell::new(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for ThreadsList {
    fn constructed(&self, obj: &Self::Type) {
        self.scrolled_window.set_parent(obj);
        // Setup
        obj.setup_model();
        obj.setup_columns();

        // imp.column_view.set_parent(&imp.window);
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.scrolled_window.unparent();
    }
}
impl WidgetImpl for ThreadsList {}
