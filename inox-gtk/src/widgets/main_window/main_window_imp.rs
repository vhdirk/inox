
use crate::core::Action;
use glib::Sender;
use adw::subclass::prelude::{AdwApplicationWindowImpl, *};
use gtk::subclass::prelude::*;
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

use crate::widgets::conversation_view::ConversationView;
use crate::widgets::conversation_list::ConversationList;

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
    pub conversation_list_box: TemplateChild<gtk::Box>,

    // menu_builder: gtk::Builder,
    pub conversation_list: OnceCell<ConversationList>,

    #[template_child]
    pub conversation_view_box: TemplateChild<gtk::Box>,

    pub conversation_view: OnceCell<ConversationView>,

    pub sender: OnceCell<Sender<Action>>
    // conversation_view: RefCell<Option<ConversationView>>, // current_notification: RefCell<Option<Rc<Notification>>>,
}

impl Default for MainWindow {
    fn default() -> Self {
        MainWindow {
            // main_header: TemplateChild::default(),
            // main_layout: TemplateChild::default(),
            mail_search: TemplateChild::default(),
            conversation_list_box: TemplateChild::default(),
            conversation_list: OnceCell::new(),

            conversation_view_box: TemplateChild::default(),
            conversation_view: OnceCell::new(),
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
        if let Some(tv) = self.conversation_view.get() {
            tv.unparent()
        }
        if let Some(tl) = self.conversation_list.get() {
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
        let conversation_list = ConversationList::new(self.sender.get().unwrap().clone());
        conversation_list.set_parent(&self.conversation_list_box.get());
        conversation_list.show();
        self.conversation_list_box.show();
        self.conversation_list
            .set(conversation_list)
            .expect("Threads list box was not empty");
        // // conversation_list.setup_signals();

        let conversation_view = ConversationView::new(self.sender.get().unwrap().clone());
        conversation_view.set_parent(&self.conversation_view_box.get());
        conversation_view.show();
        self.conversation_view_box.show();
        self.conversation_view
            .set(conversation_view)
            .expect("Thread view box was not empty");


        let inst = self.instance();
        self.mail_search.get().connect_search_changed(move |f| {
            let this = Self::from_instance(&inst);
            this.sender.get().unwrap().send(Action::Search(f.text().to_string()));
        });
        // conversation_view.setup_signals();

        // thread_box.add(&conversation_view.widget);
        // conversation_view.widget.show_all();
        // imp.conversation_view.replace(Some(conversation_view));

        // self.resize(800, 480);
    }

}
