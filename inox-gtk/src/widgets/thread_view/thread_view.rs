use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;

use notmuch;

use crate::app::Action;

const COLUMN_ID: u8 = 0;
const COLUMN_THREAD: u8 = 1;
const COLUMN_AUTHORS: u8 = 2;

mod imp {
    use crate::app::Action;
    // use crate::widgets::thread_view::thread_messages_view::ThreadMessagesView;
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::Sender;
    use gtk::{self, prelude::*, subclass::prelude::*, CompositeTemplate};
    use inox_core::database::Thread;
    use once_cell::unsync::OnceCell;
    use std::cell::RefCell;

    pub fn create_liststore() -> gio::ListStore {
        gio::ListStore::new(Thread::static_type())
    }

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Inox/gtk/thread_view.ui")]
    pub struct ThreadView {
        // Stack pages
        #[template_child]
        pub loading_page: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub no_threads_page: TemplateChild<gtk::Grid>,
        #[template_child]
        pub thread_page: TemplateChild<gtk::Grid>,
        #[template_child]
        pub multiple_threads_page: TemplateChild<gtk::Grid>,
        #[template_child]
        pub empty_folder_page: TemplateChild<gtk::Grid>,
        #[template_child]
        pub empty_search_page: TemplateChild<gtk::Grid>,
        #[template_child]
        pub composer_page: TemplateChild<gtk::Grid>,



        #[template_child]
        pub thread_find_bar: TemplateChild<gtk::SearchBar>,

        #[template_child]
        pub thread_find_entry: TemplateChild<gtk::SearchEntry>,
    // private Components.EntryUndo thread_find_undo,

        #[template_child]
        pub thread_find_next: TemplateChild<gtk::Button>,

        #[template_child]
        pub thread_find_prev: TemplateChild<gtk::Button>,


        // pub messages_view: RefCell<Option<ThreadMessagesView>>,
        pub thread_scroller: gtk::ScrolledWindow,
        //pub composer:
        pub sender: OnceCell<Sender<Action>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThreadView {
        const NAME: &'static str = "InoxThreadView";
        type Type = super::ThreadView;
        type ParentType = gtk::Widget;

        fn new() -> Self {
            let model = gio::ListStore::new(Thread::static_type());
            let thread_scroller = gtk::ScrolledWindow::builder()
                .vexpand(true)
                .hexpand(true)
                .hscrollbar_policy(gtk::PolicyType::Never)
                .build();

            thread_scroller.show();

            Self {
                loading_page: TemplateChild::default(),
                no_threads_page: TemplateChild::default(),
                thread_page: TemplateChild::default(),
                multiple_threads_page: TemplateChild::default(),
                empty_folder_page: TemplateChild::default(),
                empty_search_page: TemplateChild::default(),
                composer_page: TemplateChild::default(),

                thread_find_bar: TemplateChild::default(),
                thread_find_entry: TemplateChild::default(),
                thread_find_next: TemplateChild::default(),
                thread_find_prev: TemplateChild::default(),

                // messages_view: RefCell::new(None),
                thread_scroller,
                sender: OnceCell::new(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }
    }

    impl ObjectImpl for ThreadView {
        fn constructed(&self, obj: &Self::Type) {
            // self.list_box.set_parent(obj);
            // Setup

            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {
            // self.list_box.unparent();
        }
    }
    impl WidgetImpl for ThreadView {}
}

// Wrap imp::ThreadView into a usable gtk-rs object
glib::wrapper! {
    pub struct ThreadView(ObjectSubclass<imp::ThreadView>)
        @extends gtk::Widget;
}

// ThreadView implementation itself
impl ThreadView {
    pub fn new(sender: Sender<Action>) -> Self {
        let thread_list: Self = glib::Object::new(&[]).expect("Failed to create ThreadView");
        let imp = imp::ThreadView::from_instance(&thread_list);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on ThreadView");
        thread_list.set_vexpand(true);
        thread_list.set_vexpand_set(true);

        // thread_list.setup_callbacks();

        thread_list
    }

    fn setup_model(&self) {}

    fn setup_columns(&self) {
        let imp = imp::ThreadView::from_instance(self);
    }

    pub fn load_thread(&self, thread: notmuch::Thread) {
        let imp = imp::ThreadView::from_instance(self);
        // let model = imp::create_liststore();
        // let selection_model = SingleSelection::new(Some(&model));

        // for thread in threads {
        //     model.append(&Thread::new(thread));
        // }

        // imp.column_view.set_model(Some(&selection_model));
    }
}
