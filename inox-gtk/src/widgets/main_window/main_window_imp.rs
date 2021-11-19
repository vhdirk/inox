
use adw::subclass::prelude::{AdwApplicationWindowImpl, *};
use gtk::subclass::prelude::*;
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

use crate::widgets::thread_view::ThreadView;
use crate::widgets::threads_list::ThreadsList;

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Inox/gtk/main_window.ui")]
pub struct MainWindow {
    // #[template_child]
    // pub main_header: TemplateChild<gtk::HeaderBar>,

    // #[template_child]
    // pub main_layout: TemplateChild<gtk::Box>,

    // #[template_child]
    // pub main_paned: TemplateChild<gtk::Paned>,
    #[template_child]
    pub threads_list_box: TemplateChild<gtk::Box>,

    // menu_builder: gtk::Builder,
    pub threads_list: OnceCell<ThreadsList>,

    #[template_child]
    pub thread_view_box: TemplateChild<gtk::Box>,

    pub thread_view: OnceCell<ThreadView>,
    // thread_view: RefCell<Option<ThreadView>>, // current_notification: RefCell<Option<Rc<Notification>>>,
}

impl Default for MainWindow {
    fn default() -> Self {
        MainWindow {
            // main_header: TemplateChild::default(),
            // main_layout: TemplateChild::default(),
            // main_paned: TemplateChild::default(),
            threads_list_box: TemplateChild::default(),
            threads_list: OnceCell::new(),

            thread_view_box: TemplateChild::default(),
            thread_view: OnceCell::new(),
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
        obj.setup_all();
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(tv) = self.thread_view.get() {
            tv.unparent()
        }
        if let Some(tl) = self.threads_list.get() {
            tl.unparent()
        }
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
impl AdwApplicationWindowImpl for MainWindow {}
