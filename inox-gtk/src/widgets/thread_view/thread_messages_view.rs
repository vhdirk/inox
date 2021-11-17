// use gio::prelude::*;
// use glib::subclass::prelude::*;
// use glib::Sender;
// use glib::clone;
// use gtk::prelude::*;
// use gtk::{Application, SignalListItemFactory, SingleSelection};

// use notmuch;

// use crate::app::Action;
// // use crate::widgets::thread_list_cell_renderer::CellRendererThread;
// use inox_core::database::Thread;

// const COLUMN_ID: u8 = 0;
// const COLUMN_THREAD: u8 = 1;
// const COLUMN_AUTHORS: u8 = 2;

// mod imp {
//     use crate::app::Action;
//     use glib::subclass::prelude::*;
//     use glib::Sender;
//     use gtk;
//     use gtk::prelude::*;
//     use gtk::subclass::prelude::*;
//     use inox_core::database::Thread;
//     use once_cell::unsync::OnceCell;
//     // use crate::widgets::thread_list_cell_renderer::CellRendererThread;

//     // pub fn append_text_column(tree: &gtk::TreeView, id: i32, title: &str) {
//     //     let column = gtk::TreeViewColumn::new();
//     //     let cell = CellRendererThread::new();
//     //     column.pack_start(&cell, false);
//     //     // Association of the view's column with the model's `id` column.
//     //     column.add_attribute(&cell, "thread", id);
//     //     column.set_title(&title);
//     //     tree.append_column(&column);
//     // }

//     pub fn create_liststore() -> gio::ListStore {
//         gio::ListStore::new(Thread::static_type())
//     }

//     #[derive(Debug)]
//     pub struct ThreadMessagesView {
//         pub list_box: gtk::ListBox,
//         // pub column_view: gtk::ColumnView,
//         // pub model: gio::ListStore,
//         // pub filter: gtk::TreeModelFilter,
//         // idle_handle: RefCell<Option<glib::SourceId>>,
//         // thread_list: RefCell<Option<Threads>>,

//         // num_threads: u32,
//         // num_threads_loaded: u32
//         pub sender: OnceCell<Sender<Action>>,
//     }

//     impl Default for ThreadMessagesView {
//         fn default() -> Self {
//             let model = gio::ListStore::new(Thread::static_type());
//             let selection_model = gtk::SingleSelection::new(Some(&model));
//             let column_view = gtk::ColumnView::new(Some(&selection_model));
//             let scrolled_window = gtk::ScrolledWindow::builder()
//                 .vexpand(true)
//                 .vexpand_set(true)
//                 .child(&column_view)
//                 .build();

//             scrolled_window.show();
//             column_view.show();

//             Self {
//                 scrolled_window,
//                 column_view,
//                 model,
//                 sender: OnceCell::new(),
//             }
//         }
//     }

//     #[glib::object_subclass]
//     impl ObjectSubclass for ThreadMessagesView {
//         const NAME: &'static str = "InoxThreadMessagesView";
//         type Type = super::ThreadMessagesView;
//         type ParentType = gtk::Widget;

//         fn new() -> Self {
//             Self::default()
//         }

//         fn class_init(klass: &mut Self::Class) {
//             klass.set_layout_manager_type::<gtk::BinLayout>();
//         }
//     }

//     impl ObjectImpl for ThreadMessagesView {
//         fn constructed(&self, obj: &Self::Type) {
//             self.scrolled_window.set_parent(obj);
//             // Setup
//             obj.setup_model();
//             obj.setup_columns();

//             // imp.column_view.set_parent(&imp.window);
//             self.parent_constructed(obj);
//         }

//         fn dispose(&self, _obj: &Self::Type) {
//             self.scrolled_window.unparent();
//         }
//     }
//     impl WidgetImpl for ThreadMessagesView {}
// }

// // Wrap imp::ThreadMessagesView into a usable gtk-rs object
// glib::wrapper! {
//     pub struct ThreadMessagesView(ObjectSubclass<imp::ThreadMessagesView>)
//         @extends gtk::Widget;
// }

// // ThreadMessagesView implementation itself
// impl ThreadMessagesView {
//     pub fn new(sender: Sender<Action>) -> Self {
//         let thread_list: Self = glib::Object::new(&[]).expect("Failed to create ThreadMessagesView");
//         let imp = imp::ThreadMessagesView::from_instance(&thread_list);

//         imp.sender
//             .set(sender)
//             .expect("Failed to set sender on ThreadMessagesView");
//         thread_list.set_vexpand(true);
//         thread_list.set_vexpand_set(true);

//         thread_list.setup_callbacks();

//         thread_list
//     }

//     // ANCHOR: model
//     fn setup_model(&self) {}
//     // ANCHOR_END: model

//     // ANCHOR: setup_callbacks
//     fn setup_callbacks(&self) {
//         // Get state
//         let imp = imp::ThreadMessagesView::from_instance(self);

//         let sender = imp.sender.clone();
//         imp.column_view.connect_activate(move |column_view, position| {
//             let model = column_view.model().unwrap();
//             let thread = model
//                 .item(position)
//                 .unwrap()
//                 .downcast::<Thread>()
//                 .unwrap();

//             sender.get().unwrap().send(Action::SelectThread(Some(thread)));
//         });
//     }
//     // ANCHOR_END: setup_callbacks

//     // ANCHOR: setup_factory

//     fn setup_columns(&self) {
//         let imp = imp::ThreadMessagesView::from_instance(self);

//         imp.column_view.append_column(&self.setup_authors_column());
//         imp.column_view.append_column(&self.setup_subject_column());
//         imp.column_view.append_column(&self.setup_attachment_column());
//     }

//     fn setup_authors_column(&self) -> gtk::ColumnViewColumn {
//         let imp = imp::ThreadMessagesView::from_instance(self);

//         let factory = SignalListItemFactory::new();
//         factory.connect_setup(move |_, entry| {
//             let label = gtk::Label::new(None);
//             entry.set_child(Some(&label));
//             label.set_xalign(0.0);
//             label.show();
//         });

//         factory.connect_bind(move |_, entry| {
//             let thread = entry
//                 .item()
//                 .expect("The item has to exist.")
//                 .downcast::<Thread>()
//                 .expect("The item has to be an `Thread`.");

//             let label = entry
//                 .child()
//                 .expect("The child has to exist.")
//                 .downcast::<gtk::Label>()
//                 .expect("The child has to be a `Label`.");

//             label.set_label(&thread.authors().join(", "));
//         });

//         gtk::ColumnViewColumn::builder()
//             .title("authors")
//             .factory(&factory)
//             .build()
//     }

//     fn setup_subject_column(&self) -> gtk::ColumnViewColumn {
//         let factory = SignalListItemFactory::new();
//         factory.connect_setup(move |_, entry| {
//             let label = gtk::Label::new(None);
//             entry.set_child(Some(&label));
//             label.set_xalign(0.0);
//             label.show();
//         });

//         factory.connect_bind(move |_, entry| {
//             let thread = entry
//                 .item()
//                 .expect("The item has to exist.")
//                 .downcast::<Thread>()
//                 .expect("The item has to be an `Thread`.");

//             let label = entry
//                 .child()
//                 .expect("The child has to exist.")
//                 .downcast::<gtk::Label>()
//                 .expect("The child has to be a `Label`.");

//             label.set_label(&thread.subject());
//         });

//         gtk::ColumnViewColumn::builder()
//             .title("subject")
//             .factory(&factory)
//             .build()
//     }

//     fn setup_attachment_column(&self) -> gtk::ColumnViewColumn {
//         let icon_name = "mail-attachment-symbolic";
//         let icon = gio::ThemedIcon::new(icon_name);

//         let factory = SignalListItemFactory::new();
//         factory.connect_setup(move |_, entry| {
//             let img = gtk::ImageBuilder::new().gicon(&icon).build();
//             entry.set_child(Some(&img));
//             img.hide();
//         });

//         factory.connect_bind(move |_, entry| {
//             let thread = entry
//                 .item()
//                 .expect("The item has to exist.")
//                 .downcast::<Thread>()
//                 .expect("The item has to be an `Thread`.");

//             let img = entry
//                 .child()
//                 .expect("The child has to exist.")
//                 .downcast::<gtk::Image>()
//                 .expect("The child has to be a `Image`.");

//             if thread.has_attachment() {
//                 img.show()
//             } else {
//                 img.hide()
//             }
//         });

//         // Tell factory how to unbind `ThreadRow` from `Thread`
//         // factory.connect_unbind(move |_, entry| {
//         //     let label = entry
//         //         .child()
//         //         .expect("The child has to exist.")
//         //         .downcast::<gtk::Label>()
//         //         .expect("The child has to be a `Label`.");

//         //     label.unbind();
//         // });

//         gtk::ColumnViewColumn::builder()
//             .title("attachment")
//             .factory(&factory)
//             .build()
//     }

//     pub fn set_threads(&self, threads: notmuch::Threads) {
//         let imp = imp::ThreadMessagesView::from_instance(self);
//         let model = imp::create_liststore();
//         let selection_model = SingleSelection::new(Some(&model));

//         for thread in threads {
//             model.append(&Thread::new(thread));
//         }

//         imp.column_view.set_model(Some(&selection_model));
//     }
// }
