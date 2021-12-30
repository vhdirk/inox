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
pub struct ConversationList {
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
impl ObjectSubclass for ConversationList {
    const NAME: &'static str = "InoxConversationList";
    type Type = super::ConversationList;
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

impl ObjectImpl for ConversationList {
    fn constructed(&self, obj: &Self::Type) {
        self.scrolled_window.set_parent(obj);
        // Setup
        self.setup_columns();
        self.setup_callbacks();

        // imp.column_view.set_parent(&imp.window);
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.scrolled_window.unparent();
    }
}
impl WidgetImpl for ConversationList {}

impl ConversationList {
    pub fn setup_columns(&self) {
        self.column_view.append_column(&self.setup_authors_column());
        self.column_view.append_column(&self.setup_subject_column());
        self.column_view
            .append_column(&self.setup_attachment_column());
    }

    pub fn setup_authors_column(&self) -> gtk::ColumnViewColumn {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, entry| {
            let label = gtk::Label::new(None);
            entry.set_child(Some(&label));
            label.set_xalign(0.0);
            label.show();
        });

        factory.connect_bind(move |_, entry| {
            let thread = entry
                .item()
                .expect("The item has to exist.")
                .downcast::<Thread>()
                .expect("The item has to be an `Thread`.");

            let label = entry
                .child()
                .expect("The child has to exist.")
                .downcast::<gtk::Label>()
                .expect("The child has to be a `Label`.");

            label.set_label(&thread.data().authors().join(", "));
        });

        gtk::ColumnViewColumn::builder()
            .title("authors")
            .factory(&factory)
            .build()
    }

    pub fn setup_subject_column(&self) -> gtk::ColumnViewColumn {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, entry| {
            let label = gtk::Label::new(None);
            entry.set_child(Some(&label));
            label.set_xalign(0.0);
            label.show();
        });

        factory.connect_bind(move |_, entry| {
            let thread = entry
                .item()
                .expect("The item has to exist.")
                .downcast::<Thread>()
                .expect("The item has to be an `Thread`.");

            let label = entry
                .child()
                .expect("The child has to exist.")
                .downcast::<gtk::Label>()
                .expect("The child has to be a `Label`.");

            label.set_label(&thread.data().subject());
        });

        gtk::ColumnViewColumn::builder()
            .title("subject")
            .factory(&factory)
            .build()
    }

    pub fn setup_attachment_column(&self) -> gtk::ColumnViewColumn {
        let icon_name = "mail-attachment-symbolic";
        let icon = gio::ThemedIcon::new(icon_name);

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, entry| {
            let img = ImageBuilder::new().gicon(&icon).build();
            entry.set_child(Some(&img));
            img.hide();
        });

        factory.connect_bind(move |_, entry| {
            let thread = entry
                .item()
                .expect("The item has to exist.")
                .downcast::<Thread>()
                .expect("The item has to be an `Thread`.");

            let img = entry
                .child()
                .expect("The child has to exist.")
                .downcast::<gtk::Image>()
                .expect("The child has to be a `Image`.");

            if thread.has_attachment() {
                img.show()
            } else {
                img.hide()
            }
        });

        // Tell factory how to unbind `ThreadRow` from `Thread`
        // factory.connect_unbind(move |_, entry| {
        //     let label = entry
        //         .child()
        //         .expect("The child has to exist.")
        //         .downcast::<gtk::Label>()
        //         .expect("The child has to be a `Label`.");

        //     label.unbind();
        // });

        gtk::ColumnViewColumn::builder()
            .title("attachment")
            .factory(&factory)
            .build()
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

                let mut conversation_ids = vec![];

                while selection_iter.is_valid() {
                    let selection_val = selection_iter.value();
                    let threadw = model
                        .item(selection_val)
                        .unwrap()
                        .downcast::<Thread>()
                        .unwrap();
                    let conversation_id = threadw.data().id().to_string();
                    conversation_ids.push(conversation_id);
                    selection_iter.next();
                }

                match conversation_ids.len() {
                    0 => {
                        sender
                            .send(Action::SelectThread(None))
                            .expect("Failed to send thread selected action");
                    }
                    1 => {
                        debug!("Selected thread {:?}", conversation_ids[0].clone());

                        sender
                            .send(Action::SelectThread(Some(conversation_ids[0].clone())))
                            .expect("Failed to send thread selected action");
                    }
                    _ => {
                        sender
                            .send(Action::SelectThreads(conversation_ids))
                            .expect("Failed to send thread selected action");
                    }
                };
            }
        ));
    }
}
