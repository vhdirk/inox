use crate::core::Action;
use crate::core::Thread;
use chrono::naive::NaiveDateTime;
use chrono::DateTime;
use chrono::Utc;
use chrono_humanize::HumanTime;
use glib::subclass::prelude::*;
use glib::{clone, Sender};
use gtk;
use gtk::builders::ImageBuilder;
use gtk::SignalListItemFactory;
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};
use log::*;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

pub fn create_liststore() -> gio::ListStore {
    gio::ListStore::new(Thread::static_type())
}

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Inox/gtk/thread_list_item.ui")]
pub struct ThreadListItem {
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
impl ObjectSubclass for ThreadListItem {
    const NAME: &'static str = "InoxThreadListItem";
    type Type = super::ThreadListItem;
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
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ThreadListItem {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {}
}
impl WidgetImpl for ThreadListItem {}

impl BoxImpl for ThreadListItem {}

impl ThreadListItem {
    pub fn update(&self) {
        if let Some(thread) = self.thread.borrow().as_ref() {
            self.authors_label
                .set_text(&thread.data().authors().join(", "));
            self.subject_label.set_text(&thread.data().subject());

            self.date_label.set_text(&self.format_date());

            self.num_messages_label
                .set_text(&format!("{}", thread.data().total_messages()));


            if thread.is_unread() {
                self.authors_label.style_context().add_class("inox-unread");
                self.subject_label.style_context().add_class("inox-unread");
            }
        }
    }

    pub fn format_date(&self) -> String {
        if let Some(thread) = self.thread.borrow().as_ref() {
            let dt = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(thread.data().newest_date(), 0),
                Utc,
            );
            let ht = HumanTime::from(dt);
            format!("{}", ht)
        } else {
            "".to_string()
        }
    }
}
