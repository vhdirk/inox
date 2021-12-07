use chrono::Utc;
use chrono::DateTime;
use std::cell::RefCell;
use crate::core::Action;
use crate::core::Thread;
use glib::subclass::prelude::*;
use glib::{clone, Sender};
use gtk;
use gtk::builders::ImageBuilder;
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};
use gtk::SignalListItemFactory;
use once_cell::unsync::OnceCell;
use log::*;
use chrono_humanize::HumanTime;
use chrono::naive::NaiveDateTime;

pub fn create_liststore() -> gio::ListStore {
    gio::ListStore::new(Thread::static_type())
}

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Inox/gtk/threads_list_item.ui")]
pub struct ThreadsListItem {
    pub thread: RefCell<Option<Thread>>,

    #[template_child]
    pub authors_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub subject_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub date_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub num_messages_label: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for ThreadsListItem {
    const NAME: &'static str = "InoxThreadsListItem";
    type Type = super::ThreadsListItem;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            thread: RefCell::new(None),
            authors_label: TemplateChild::default(),
            subject_label: TemplateChild::default(),
            date_label: TemplateChild::default(),
            num_messages_label: TemplateChild::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        // klass.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ThreadsListItem {
    fn constructed(&self, obj: &Self::Type) {
        // Setup

        // imp.column_view.set_parent(&imp.window);

        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
    }
}
impl WidgetImpl for ThreadsListItem {}

impl BoxImpl for ThreadsListItem {}

impl ThreadsListItem {

    pub fn update(&self) {
        if let Some(thread) = self.thread.borrow().as_ref() {
            self.authors_label.set_text(&thread.data().authors().join(", "));
            self.subject_label.set_text(&thread.data().subject());

            let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(thread.data().newest_date(), 0), Utc);
            let ht = HumanTime::from(dt);
            self.date_label.set_text(&format!("{}", ht));

            self.num_messages_label.set_text(&format!("{}", thread.data().total_messages()));
        }
    }

}
