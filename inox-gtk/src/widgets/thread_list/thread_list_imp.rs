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

use super::ThreadListItem;

pub fn create_liststore() -> gio::ListStore {
    gio::ListStore::new(Thread::static_type())
}

#[derive(Debug)]
pub struct ThreadList {
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

#[glib::object_subclass]
impl ObjectSubclass for ThreadList {
    const NAME: &'static str = "InoxThreadList";
    type Type = super::ThreadList;
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

impl ObjectImpl for ThreadList {
    fn constructed(&self, obj: &Self::Type) {
        self.scrolled_window.set_parent(obj);
        // Setup
        self.setup_column();
        self.setup_callbacks();

        // imp.column_view.set_parent(&imp.window);
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.scrolled_window.unparent();
    }
}
impl WidgetImpl for ThreadList {}

impl ThreadList {
    pub fn setup_column(&self) {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, entry| {
            let item = ThreadListItem::new();
            entry.set_child(Some(&item));
            item.show();
        });

        factory.connect_bind(move |_, entry| {
            let thread = entry
                .item()
                .expect("The item has to exist.")
                .downcast::<Thread>()
                .expect("The item has to be an `Thread`.");

            let item = entry
                .child()
                .expect("The child has to exist.")
                .downcast::<ThreadListItem>()
                .expect("The child has to be a `ThreadListItem`.");

            item.set_thread(&thread);
        });

        let column = gtk::ColumnViewColumn::builder()
            .factory(&factory)
            .build();

        self.column_view.append_column(&column);
    }

    // ANCHOR: setup_callbacks
    pub fn setup_callbacks(&self) {
        // Get state
        let inst = self.instance().clone();

        self.column_view.model().unwrap().connect_selection_changed(
            clone!(@weak inst => move |model, position, n_items| {
                let this = Self::from_instance(&inst);
                let sender = this.sender.get().unwrap();
                // TODO: is selection_in_range the best choice here?
                let selection = model.selection_in_range(position, n_items);
                let (mut selection_iter, _) = gtk::BitsetIter::init_first(&selection).unwrap();

                let mut thread_ids = vec![];

                while selection_iter.is_valid() {
                    let selection_val = selection_iter.value();
                    let threadw = model
                        .item(selection_val)
                        .unwrap()
                        .downcast::<Thread>()
                        .unwrap();
                    let thread_id = threadw.data().id().to_string();
                    thread_ids.push(thread_id);
                    selection_iter.next();
                }

                match thread_ids.len() {
                    0 => {
                        sender
                            .send(Action::SelectThread(None))
                            .expect("Failed to send thread selected action");
                    }
                    1 => {
                        debug!("Selected thread {:?}", thread_ids[0].clone());

                        sender
                            .send(Action::SelectThread(Some(thread_ids[0].clone())))
                            .expect("Failed to send thread selected action");
                    }
                    _ => {
                        sender
                            .send(Action::SelectThreads(thread_ids))
                            .expect("Failed to send thread selected action");
                    }
                };
            }
        ));
    }
}
