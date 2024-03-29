use crate::core::Action;
use crate::widgets::placeholder_pane::PlaceholderPane;
// use crate::widgets::thread_view::message_list::MessageList;
use crate::widgets::MessageList;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::{self, prelude::*, subclass::prelude::*, CompositeTemplate};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

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

    pub no_threads_placeholder: PlaceholderPane,
    pub multi_threads_placeholder: PlaceholderPane,
    pub empty_tag_placeholder: PlaceholderPane,
    pub empty_search_placeholder: PlaceholderPane,

    pub message_list: RefCell<Option<MessageList>>,
    pub thread_scroller: gtk::ScrolledWindow,
    //pub composer:
    pub sender: OnceCell<Sender<Action>>,
}

impl ThreadView {
    pub fn set_visible_child<W: IsA<gtk::Widget>>(&self, widget: &W) {
        let current = self.stack.get().visible_child();

        if current.is_some()
            && self
                .thread_page
                .get()
                .upcast::<gtk::Widget>()
                .eq(current.as_ref().unwrap())
        {
            if self.thread_page.get().upcast::<gtk::Widget>().eq(widget) {
                // By removing the current list, any load it is currently
                // performing is also cancelled, which is important to
                // avoid a possible crit warning when switching folders,
                // etc.
                // self.remove_message_list();
            }
        } else if current.is_some()
            && self
                .loading_page
                .get()
                .upcast::<gtk::Widget>()
                .eq(current.as_ref().unwrap())
        {
            // Stop the spinner running so it doesn't trigger repaints
            // and wake up Inox even when idle.
            self.loading_page.get().stop();
        }
        self.stack.get().set_visible_child(widget);
    }

    //add_new_list
    pub fn set_message_list(&self, list: &MessageList) {

        self.remove_message_list();

        *self.message_list.borrow_mut() = Some(list.clone());

        // // Manually create a Viewport rather than letting
        // // ScrolledWindow do it so Container.set_focus_{h,v}adjustment
        // // are not set on the list - it makes changing focus jumpy
        // // when a row or its web_view are larger than the viewport.
        // let viewport = gtk::Viewport::builder().build();
        // viewport.show();
        // viewport.set_child(Some(list));

        self.thread_scroller.set_child(Some(list));


    }

    // Remove any existing thread list, cancelling its loading
    // remove_current_list
    pub fn remove_message_list(&self) {
        // do not unparent the list from the scrolled window. It does that by itself
        // when setting the new child
        *self.message_list.borrow_mut() = None;

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

            no_threads_placeholder: PlaceholderPane::new(
                "folder-symbolic",
                "No threads selected",
                "Selecting a thread from the list will display it here",
            ),
            multi_threads_placeholder: PlaceholderPane::new(
                "folder-symbolic",
                "Multiple threads selected",
                "Choosing an action will apply to all selected threads",
            ),
            empty_tag_placeholder: PlaceholderPane::new(
                "folder-symbolic",
                "No threads found",
                "This tag has not been applied to any threads",
            ),
            empty_search_placeholder: PlaceholderPane::new(
                "folder-symbolic",
                "No threads found",
                "Your search returned no results, try refining your search terms",
            ),

            message_list: RefCell::new(None),
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
        self.no_threads_placeholder
            .set_parent(&self.no_threads_page.get());
        self.multi_threads_placeholder
            .set_parent(&self.multiple_threads_page.get());
        self.empty_tag_placeholder
            .set_parent(&self.empty_tag_page.get());
        self.empty_search_placeholder
            .set_parent(&self.empty_search_page.get());

        self.thread_scroller.set_parent(&self.thread_page.get());
        self.parent_constructed(obj);

        self.thread_scroller.show();
    }

    fn dispose(&self, _obj: &Self::Type) {
        self.no_threads_placeholder.unparent();
        self.multi_threads_placeholder.unparent();
        self.empty_tag_placeholder.unparent();
        self.empty_search_placeholder.unparent();

        self.remove_message_list();
        self.thread_scroller.unparent();
    }
}
impl WidgetImpl for ThreadView {}
