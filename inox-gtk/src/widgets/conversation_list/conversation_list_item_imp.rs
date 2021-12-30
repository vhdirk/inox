use inox_core::models::Conversation;
use crate::core::Action;
use crate::core::ConversationObject;
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

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Inox/gtk/conversation_list_item.ui")]
pub struct ConversationListItem {
    pub conversation: RefCell<Option<ConversationObject>>,

    #[template_child]
    pub authors_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub subject_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub date_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub tags_container: TemplateChild<gtk::Box>,

    #[template_child]
    pub num_messages_label: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for ConversationListItem {
    const NAME: &'static str = "InoxConversationListItem";
    type Type = super::ConversationListItem;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            conversation: RefCell::new(None),
            authors_label: TemplateChild::default(),
            subject_label: TemplateChild::default(),
            date_label: TemplateChild::default(),
            num_messages_label: TemplateChild::default(),
            tags_container: TemplateChild::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ConversationListItem {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {}
}
impl WidgetImpl for ConversationListItem {}

impl BoxImpl for ConversationListItem {}

impl ConversationListItem {
    pub fn update(&self) {
        if let Some(conversation) = self.conversation.borrow().as_ref() {
            // TODO
            // self.authors_label
            //     .set_text(&conversation.authors.join(", "));
            self.subject_label.set_text(&conversation.subject);

            self.date_label.set_text(&self.format_date());

            self.num_messages_label
                .set_text(&format!("{}", conversation.total_messages));

            if conversation.is_unread() {
                self.authors_label.style_context().add_class("inox-unread");
                self.subject_label.style_context().add_class("inox-unread");
            }
        }
    }

    pub fn update_tags(&self) {
        if let Some(conversation) = self.conversation.borrow().as_ref() {
            let container = self.tags_container.get();
            for tag in conversation.tags.iter() {
                let tag_label = gtk::Label::new(Some(&tag));
                container.append(&tag_label);
            }

            let tag_label = gtk::Label::new(Some(&"lalala"));
            container.append(&tag_label);
        }
    }

    pub fn format_date(&self) -> String {
        if let Some(conversation) = self.conversation.borrow().as_ref() {
            // let dt = DateTime::<Utc>::from_utc(
            //     NaiveDateTime::from_timestamp(newest_date, 0),
            //     Utc,
            // );
            let ht = HumanTime::from(conversation.newest_date);
            return format!("{}", ht);

        }
        "".to_string()
    }
}
