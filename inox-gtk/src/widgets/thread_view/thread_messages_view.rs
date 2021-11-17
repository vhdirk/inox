use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::{Application, SignalListItemFactory, SingleSelection};

use notmuch;

use crate::app::Action;
// use crate::widgets::thread_list_cell_renderer::CellRendererThread;
use inox_core::database::Thread;

mod imp {
    use crate::app::Action;
    use glib::subclass::prelude::*;
    use glib::Sender;
    use gtk;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use inox_core::database::Thread;
    use once_cell::unsync::OnceCell;

    #[derive(Debug)]
    pub struct ThreadMessagesView {
        // pub list_box: gtk::ListBox,
        // pub column_view: gtk::ColumnView,
        // pub model: gio::ListStore,
        // pub filter: gtk::TreeModelFilter,
        // idle_handle: RefCell<Option<glib::SourceId>>,
        // thread_list: RefCell<Option<Threads>>,

        // num_threads: u32,
        // num_threads_loaded: u32
        pub sender: OnceCell<Sender<Action>>,
    }

    impl Default for ThreadMessagesView {
        fn default() -> Self {
            Self {
                sender: OnceCell::new(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThreadMessagesView {
        const NAME: &'static str = "InoxThreadMessagesView";
        type Type = super::ThreadMessagesView;
        type ParentType = gtk::Widget;

        fn new() -> Self {
            Self::default()
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }
    }

    impl ObjectImpl for ThreadMessagesView {
        fn constructed(&self, obj: &Self::Type) {
            // imp.column_view.set_parent(&imp.window);
            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {}
    }
    impl WidgetImpl for ThreadMessagesView {}
}

// Wrap imp::ThreadMessagesView into a usable gtk-rs object
glib::wrapper! {
    pub struct ThreadMessagesView(ObjectSubclass<imp::ThreadMessagesView>)
        @extends gtk::Widget;
}

// ThreadMessagesView implementation itself
impl ThreadMessagesView {
    pub fn new(thread: &Thread, sender: Sender<Action>) -> Self {
        let thread_list: Self =
            glib::Object::new(&[]).expect("Failed to create ThreadMessagesView");
        let imp = imp::ThreadMessagesView::from_instance(&thread_list);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on ThreadMessagesView");
        thread_list.set_vexpand(true);
        thread_list.set_vexpand_set(true);

        thread_list
    }
}
