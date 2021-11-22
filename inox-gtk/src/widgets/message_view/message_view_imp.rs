use crate::core::Action;
use crate::core::Message;
use chrono_humanize::HumanTime;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;
use gmime::traits::MessageExt;
use gmime::{InternetAddressExt, InternetAddressListExt};

use gtk::{self, prelude::*, subclass::prelude::*, CompositeTemplate};
use once_cell::unsync::OnceCell;

const EMPTY_FROM_LABEL: &str = "No sender";

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Inox/gtk/message_view.ui")]
pub struct MessageView {
    #[template_child]
    pub actions: TemplateChild<gtk::Grid>,

    #[template_child]
    pub attachments_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub star_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub unstar_button: TemplateChild<gtk::Button>,

    // #[template_child]
    // pub email_menubutton: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub avatar: TemplateChild<adw::Avatar>,

    #[template_child]
    pub message_container: TemplateChild<gtk::Grid>,

    #[template_child]
    pub compact_revealer: TemplateChild<gtk::Revealer>,
    #[template_child]
    pub compact_from: TemplateChild<gtk::Label>,
    #[template_child]
    pub compact_date: TemplateChild<gtk::Label>,
    #[template_child]
    pub compact_body: TemplateChild<gtk::Label>,

    #[template_child]
    pub header_revealer: TemplateChild<gtk::Revealer>,
    #[template_child]
    pub from: TemplateChild<gtk::FlowBox>,
    #[template_child]
    pub subject: TemplateChild<gtk::Label>,
    #[template_child]
    pub date: TemplateChild<gtk::Label>,

    #[template_child]
    pub sender_header: TemplateChild<gtk::Grid>,
    #[template_child]
    pub sender_address: TemplateChild<gtk::FlowBox>,

    #[template_child]
    pub reply_to_header: TemplateChild<gtk::Grid>,
    #[template_child]
    pub reply_to_addresses: TemplateChild<gtk::FlowBox>,

    #[template_child]
    pub to_header: TemplateChild<gtk::Grid>,
    #[template_child]
    pub cc_header: TemplateChild<gtk::Grid>,
    #[template_child]
    pub bcc_header: TemplateChild<gtk::Grid>,

    #[template_child]
    pub body_revealer: TemplateChild<gtk::Revealer>,
    #[template_child]
    pub body_progress: TemplateChild<gtk::ProgressBar>,

    pub message: OnceCell<Message>,

    pub sender: OnceCell<Sender<Action>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageView {
    const NAME: &'static str = "InoxMessageView";
    type Type = super::MessageView;
    type ParentType = gtk::Widget;

    fn new() -> Self {
        Self {
            actions: TemplateChild::default(),
            attachments_button: TemplateChild::default(),
            star_button: TemplateChild::default(),
            unstar_button: TemplateChild::default(),
            // email_menubutton: TemplateChild::default(),
            avatar: TemplateChild::default(),

            message_container: TemplateChild::default(),

            compact_revealer: TemplateChild::default(),
            compact_from: TemplateChild::default(),
            compact_date: TemplateChild::default(),
            compact_body: TemplateChild::default(),

            header_revealer: TemplateChild::default(),
            from: TemplateChild::default(),
            subject: TemplateChild::default(),
            date: TemplateChild::default(),

            sender_header: TemplateChild::default(),
            sender_address: TemplateChild::default(),

            reply_to_header: TemplateChild::default(),
            reply_to_addresses: TemplateChild::default(),

            to_header: TemplateChild::default(),
            cc_header: TemplateChild::default(),
            bcc_header: TemplateChild::default(),

            body_revealer: TemplateChild::default(),
            body_progress: TemplateChild::default(),

            message: OnceCell::new(),

            sender: OnceCell::new(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MessageView {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    fn dispose(&self, obj: &Self::Type) {
        // TODO: not sure why we need to unparent these manually?
        self.actions.get().unparent();
        self.message_container.get().unparent();
    }
}
impl WidgetImpl for MessageView {}

impl MessageView {
    pub fn update_compact(&self) {
        self.compact_body
            .get()
            .set_text(&self.format_body_compact());
        self.compact_from
            .get()
            .set_text(&self.format_originator_compact());
        self.compact_date
            .get()
            .set_text(&self.format_date_compact());
    }

    pub fn update_expanded(&self) {
        let msg = self.message.get().unwrap();

        if let Some(subject) = msg.subject() {
            self.subject.get().set_text(&subject);
        }

    }

    pub fn format_originator_compact(&self) -> String {
        let msg = self.message.get().unwrap();
        let from = msg.from();

        if from.is_none() {
            return EMPTY_FROM_LABEL.to_string();
        }

        let from = from.unwrap();
        let num_from = from.length();

        let mut originators = vec![];
        for i in 0..num_from {
            // TODO: link email addresses to addressbook
            let from_address = from.address(i);
            if from_address.is_none() {
                continue;
            }

            let from_name = from_address.unwrap().name();
            if from_name.is_none() {
                continue;
            }

            originators.push(from_name.unwrap().to_string());
        }

        originators.join(", ")
    }

    pub fn format_date_compact(&self) -> String {
        let msg = self.message.get().unwrap();
        let ht = HumanTime::from(msg.date());
        format!("{}", ht)
    }

    pub fn format_body_compact(&self) -> String {
        let msg = self.message.get().unwrap();
        msg.preview()
    }
}
