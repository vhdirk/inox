
use crate::core::Action;
use crate::core::Message;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;
use gmime::traits::MessageExt;
use gtk::{self, prelude::*, subclass::prelude::*, CompositeTemplate};
use once_cell::unsync::OnceCell;

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

    #[template_child]
    pub email_menubutton: TemplateChild<gtk::MenuButton>,

    #[template_child]
    pub avatar: TemplateChild<adw::Avatar>,

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

    pub message: OnceCell<notmuch::Message>,
    pub parsed_message: OnceCell<Message>,

    pub sender: OnceCell<Sender<Action>>,
}

impl MessageView {
    pub fn update(&self) {
        let message = self.message.get();
        let parsed = self.parsed_message.get();

        self.subject
            .get()
            .set_text(&parsed.unwrap().subject().unwrap());
        self.compact_body
            .get()
            .set_text(&parsed.unwrap().subject().unwrap());
    }
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
            email_menubutton: TemplateChild::default(),

            avatar: TemplateChild::default(),

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
            parsed_message: OnceCell::new(),

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

    fn dispose(&self, _obj: &Self::Type) {
        // self.list_box.unparent();
    }
}
impl WidgetImpl for MessageView {}
