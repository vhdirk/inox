use gtk::SingleSelection;
use gio::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;

use notmuch;

use crate::app::Action;
use crate::widgets::thread_list_cell_renderer::CellRendererThread;
use inox_core::database::Thread;

const COLUMN_ID: u8 = 0;
const COLUMN_THREAD: u8 = 1;
const COLUMN_AUTHORS: u8 = 2;

mod imp {
    use glib::subclass::prelude::*;
    use glib::Sender;
    use gtk as gtk;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use inox_core::database::Thread;
    use crate::widgets::thread_list_cell_renderer::CellRendererThread;

    pub fn append_text_column(tree: &gtk::TreeView, id: i32, title: &str) {
        let column = gtk::TreeViewColumn::new();
        let cell = CellRendererThread::new();
        column.pack_start(&cell, false);
        // Association of the view's column with the model's `id` column.
        column.add_attribute(&cell, "thread", id);
        column.set_title(&title);
        tree.append_column(&column);
    }


    pub fn create_liststore() -> gio::ListStore {
        gio::ListStore::new(Thread::static_type())
    }

    #[derive(Debug, Default)]
    pub struct ThreadList {
        pub scrolled_window: gtk::ScrolledWindow,
        pub column_view: gtk::ColumnView,
        // pub filter: gtk::TreeModelFilter,
        // idle_handle: RefCell<Option<glib::SourceId>>,
        // thread_list: RefCell<Option<Threads>>,

        // num_threads: u32,
        // num_threads_loaded: u32
        // pub sender: Sender<Action>,
    }

    // impl Default for ThreadList {
    //     fn default() -> Self {
    //         let window = gtk::ScrolledWindow::new();
    //         let model = create_liststore();
    //         let filter = gtk::TreeModelFilter::new(&model, None);
    //         let tree = gtk::TreeView::new();
    //         // window.set_child(Some(&tree));

    //         Self {
    //             window,
    //             tree,
    //             filter,
    //         }
    //     }
    // }

    #[glib::object_subclass]
    impl ObjectSubclass for ThreadList {
        const NAME: &'static str = "inox_ThreadList";
        type Type = super::ThreadList;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for ThreadList {
        fn constructed(&self, obj: &Self::Type) {
            let imp = ThreadList::from_instance(obj);
            imp.scrolled_window.set_parent(obj);
            imp.scrolled_window.set_child(Some(&imp.column_view));
            // imp.column_view.set_parent(&imp.window);
            self.parent_constructed(obj);
        }

        fn dispose(&self, obj: &Self::Type) {
            let imp = ThreadList::from_instance(obj);
            imp.scrolled_window.unparent();

        }
    }
    impl WidgetImpl for ThreadList {}
}



// Wrap imp::ThreadList into a usable gtk-rs object
glib::wrapper! {
    pub struct ThreadList(ObjectSubclass<imp::ThreadList>)
        @extends gtk::Widget;
}

// ThreadList implementation itself
impl ThreadList {
    pub fn new(sender: Sender<Action>) -> Self {
        let thread_list: Self = glib::Object::new(&[]).expect("Failed to create ThreadList");

        thread_list
    }

    pub fn set_threads(&self, threads: notmuch::Threads) {
        let imp = imp::ThreadList::from_instance(self);
        let model = imp::create_liststore();
        let selection_model = SingleSelection::new(Some(&model));

        for thread in threads {
            self.add_thread(&model, Thread::new(thread));
        }

        imp.column_view.set_model(Some(&selection_model));
    }

    fn add_thread(&self, model: &gio::ListStore, thread: Thread) {
        let imp = imp::ThreadList::from_instance(self);

        dbg!("add thread {:?}", thread.id());

        let thread_id = &(*thread.id()).to_owned();
        model.append(&thread);
    }
}
