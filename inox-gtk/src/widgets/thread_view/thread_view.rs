use inox_core::database::Thread;
use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;

use notmuch;

use crate::app::Action;
use super::thread_messages_view::ThreadMessagesView;

mod imp {
    use crate::app::Action;
    use crate::widgets::placeholder_pane::PlaceholderPane;
    // use crate::widgets::thread_view::thread_messages_view::ThreadMessagesView;
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::Sender;
    use gtk::{self, prelude::*, subclass::prelude::*, CompositeTemplate};
    use once_cell::unsync::OnceCell;
    use super::ThreadMessagesView;


    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Inox/gtk/thread_view.ui")]
    pub struct ThreadView {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
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
        pub empty_tag_page: TemplateChild<gtk::Grid>,
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

    impl ThreadView {
        pub fn set_visible_child<W: IsA<gtk::Widget>>(&self, widget: &W) {
            let current = self.stack.get().visible_child();
            // if current == self.thread_page) {
            //     if (widget != self.thread_page) {
            //         // By removing the current list, any load it is currently
            //         // performing is also cancelled, which is important to
            //         // avoid a possible crit warning when switching folders,
            //         // etc.
            //         self.remove_current_list();
            //     }
            // } else if (current == self.loading_page) {
            //     // Stop the spinner running so it doesn't trigger repaints
            //     // and wake up Geary even when idle. See Bug 783025.
            //     self.loading_page.stop();
            // }
            self.stack.get().set_visible_child(widget);
        }

        //add_new_list
        pub fn set_messages_view(&self, list: &ThreadMessagesView) {

            // this.current_list = list;
            list.show();

            // // Manually create a Viewport rather than letting
            // // ScrolledWindow do it so Container.set_focus_{h,v}adjustment
            // // are not set on the list - it makes changing focus jumpy
            // // when a row or its web_view are larger than the viewport.
            // Gtk.Viewport viewport = new Gtk.Viewport(null, null);
            // viewport.show();
            // viewport.add(list);

            // self.thread_scroller.add(viewport);
            list.set_parent(&self.thread_scroller);
        }


        // Remove any existing thread list, cancelling its loading
        // remove_current_list
        pub fn remove_messages_view(&self) {
            // if (self.find_cancellable != null) {
            //     self.find_cancellable.cancel();
            //     self.find_cancellable = null;
            // }

            // if (self.current_list != null) {
            //     self.current_list.cancel_thread_load();
            //     self.thread_removed(self.current_list);
            //     this.current_list = null;
            // }

            // var old_scroller = this.thread_scroller;
            // // XXX GTK+ Bug 778190 workaround
            // this.thread_page.remove(old_scroller);
            // new_thread_scroller();
            // return old_scroller;
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThreadView {
        const NAME: &'static str = "InoxThreadView";
        type Type = super::ThreadView;
        type ParentType = gtk::Widget;

        fn new() -> Self {
            // let model = gio::ListStore::new(Thread::static_type());
            let thread_scroller = gtk::ScrolledWindow::builder()
                .vexpand(true)
                .hexpand(true)
                .hscrollbar_policy(gtk::PolicyType::Never)
                .build();

            // thread_scroller.show();

            Self {
                stack: TemplateChild::default(),

                loading_page: TemplateChild::default(),
                no_threads_page: TemplateChild::default(),
                thread_page: TemplateChild::default(),
                multiple_threads_page: TemplateChild::default(),
                empty_tag_page: TemplateChild::default(),
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
            Self::bind_template(klass);
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ThreadView {
        fn constructed(&self, obj: &Self::Type) {
            let no_threads = PlaceholderPane::new(
                "folder-symbolic",
                "No threads selected",
                "Selecting a thread from the list will display it here",
            );
            no_threads.set_parent(&self.no_threads_page.get());

            let multi_threads = PlaceholderPane::new(
                "folder-symbolic",
                "Multiple threads selected",
                "Choosing an action will apply to all selected threads",
            );
            multi_threads.set_parent(&self.multiple_threads_page.get());

            let empty_tag = PlaceholderPane::new(
                "folder-symbolic",
                "No threads found",
                "This tag has not been applied to any threads",
            );
            empty_tag.set_parent(&self.empty_tag_page.get());

            let empty_search = PlaceholderPane::new(
                "folder-symbolic",
                "No threads found",
                "Your search returned no results, try refining your search terms",
            );
            empty_search.set_parent(&self.empty_search_page.get());

            self.thread_scroller.set_parent(obj);
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

    /**
     * Shows the loading UI.
     */
    fn show_loading(&self) {
        let imp = imp::ThreadView::from_instance(self);

        imp.loading_page.get().start();
        imp.set_visible_child(&imp.loading_page.get());
    }

    fn setup_columns(&self) {
        let imp = imp::ThreadView::from_instance(self);
    }

    pub fn load_thread(&self, thread: &Thread) {
        let imp = imp::ThreadView::from_instance(self);
        // self.show_loading();

        let messages_view = ThreadMessagesView::new(thread, imp.sender.get().unwrap().clone());


        imp.set_messages_view(&messages_view);
        imp.set_visible_child(&imp.thread_page.get());


        // let model = imp::create_liststore();
        // let selection_model = SingleSelection::new(Some(&model));

        // for thread in threads {
        //     model.append(&Thread::new(thread));
        // }

        // imp.column_view.set_model(Some(&selection_model));
    }
}
