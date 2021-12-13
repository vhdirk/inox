
use crate::core::Action;
use glib::Sender;
use adw::subclass::prelude::{AdwApplicationWindowImpl, *};
use gtk::subclass::prelude::*;
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

use crate::widgets::thread_view::ThreadView;
use crate::widgets::thread_list::ThreadList;

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Inox/gtk/main_window.ui")]
pub struct MainWindow {
    // #[template_child]
    // pub main_header: TemplateChild<gtk::HeaderBar>,

    // #[template_child]
    // pub main_layout: TemplateChild<gtk::Box>,

    #[template_child]
    pub mail_search: TemplateChild<gtk::SearchEntry>,

    #[template_child]
    pub thread_list_box: TemplateChild<gtk::Box>,

    // menu_builder: gtk::Builder,
    pub thread_list: OnceCell<ThreadList>,

    #[template_child]
    pub thread_view_box: TemplateChild<gtk::Box>,

    pub thread_view: OnceCell<ThreadView>,

    pub sender: OnceCell<Sender<Action>>
    // thread_view: RefCell<Option<ThreadView>>, // current_notification: RefCell<Option<Rc<Notification>>>,
}

impl Default for MainWindow {
    fn default() -> Self {
        MainWindow {
            // main_header: TemplateChild::default(),
            // main_layout: TemplateChild::default(),
            mail_search: TemplateChild::default(),
            thread_list_box: TemplateChild::default(),
            thread_list: OnceCell::new(),

            thread_view_box: TemplateChild::default(),
            thread_view: OnceCell::new(),
            sender: OnceCell::new()
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "InoxMainWindow";
    type Type = super::MainWindow;
    type ParentType = adw::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Implement GLib.Object for MainWindow
impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(tv) = self.thread_view.get() {
            tv.unparent()
        }
        if let Some(tl) = self.thread_list.get() {
            tl.unparent()
        }
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
impl AdwApplicationWindowImpl for MainWindow {}

impl MainWindow {

    pub fn init(&self) {
        let thread_list = ThreadList::new(self.sender.get().unwrap().clone());
        thread_list.set_parent(&self.thread_list_box.get());
        thread_list.show();
        self.thread_list_box.show();
        self.thread_list
            .set(thread_list)
            .expect("Threads list box was not empty");
        // // thread_list.setup_signals();

        let thread_view = ThreadView::new(self.sender.get().unwrap().clone());
        thread_view.set_parent(&self.thread_view_box.get());
        thread_view.show();
        self.thread_view_box.show();
        self.thread_view
            .set(thread_view)
            .expect("Thread view box was not empty");


        let inst = self.instance();
        self.mail_search.get().connect_search_changed(move |f| {
            let this = Self::from_instance(&inst);
            this.sender.get().unwrap().send(Action::Search(f.text().to_string()));
        });
        // thread_view.setup_signals();

        // thread_box.add(&thread_view.widget);
        // thread_view.widget.show_all();
        // imp.thread_view.replace(Some(thread_view));

        // self.resize(800, 480);
    }

}
