use std::cell::RefCell;

use crate::core::Action;
use crate::core::Message;
use crate::widgets::web_view::MessageWebView;
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
    #[template_child]
    pub body_container: TemplateChild<gtk::Grid>,

    pub body_placeholder: RefCell<Option<gtk::Widget>>,

    pub web_view: OnceCell<MessageWebView>,

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
            body_container: TemplateChild::default(),
            body_placeholder: RefCell::new(None),

            web_view: OnceCell::new(),

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
    pub fn update_collapsed(&self) {
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

    /**
     * Shows the complete message: headers, body and attachments.
     */
    pub fn show_message_body(&self, include_transitions: bool) {
        if self.web_view.get().is_none() {
            self.initialize_web_view();
        }

        self.set_revealer(&self.compact_revealer.get(), false, include_transitions);
        self.set_revealer(&self.header_revealer.get(), true, include_transitions);
        self.set_revealer(&self.body_revealer.get(), true, include_transitions);
    }

    pub fn hide_message_body(&self) {
        self.compact_revealer.get().set_reveal_child(true);
        self.header_revealer.get().set_reveal_child(false);
        self.body_revealer.get().set_reveal_child(false);
    }

    pub fn initialize_web_view(&self) -> MessageWebView {
        let web_view = MessageWebView::new(self.sender.get().unwrap().clone());
        // web_view.set_parent(&self.body_container.get());
        self.body_container.get().show();
        self.body_container.get().set_vexpand(true);
        self.body_container.get().set_hexpand(true);
        self.body_container.get().attach(&web_view, 0, 0, 1, 1);

        web_view
    }

    /**
     * Starts loading the message body in the HTML view.
     */
    pub fn load_message_body(&self) {
        // throws GLib.Error {
        // if (load_cancelled.is_cancelled()) {
        //     throw new GLib.IOError.CANCELLED("Conversation load cancelled");
        // }

        self.web_view.get_or_init(move || {
            self.initialize_web_view()
        });


        // bool contact_load_images = (
        //     this.primary_contact != null &&
        //     this.primary_contact.load_remote_resources
        // );
        // if (this.load_remote_resources || contact_load_images) {
        //     yield this.web_view.load_remote_resources(load_cancelled);
        // }

        self.show_placeholder_pane(None);

        let body_text = if let Some(msg) = self.message.get() {
            if msg.has_html_body() {
                msg.html_body(/*inline_image_replacer*/)
            } else {
                msg.plain_body(true /*inline_image_replacer*/)
            }
        } else {
            None
        };

        dbg!("Body text: {:?}", &body_text);

        // load_cancelled.cancelled.connect(() => { web_view.stop_loading(); });
        self.web_view
            .get()
            .unwrap()
            .load_html(&body_text.unwrap_or_else(|| "".to_string()));
    }

    pub fn show_placeholder_pane(&self, placeholder: Option<&gtk::Widget>) {
        if let Some(placeholder) = &*self.body_placeholder.borrow() {
            placeholder.hide();
            self.body_container.get().remove(placeholder);
        }

        *self.body_placeholder.borrow_mut() = placeholder.cloned();

        if let Some(placeholder) = placeholder {
            if let Some(web_view) = self.web_view.get() {
                web_view.hide();
            }
            placeholder.set_parent(&self.body_container.get());
            self.show_message_body(true);
        } else if let Some(web_view) = self.web_view.get() {
            web_view.show();
        }
    }

    pub fn set_revealer(&self, revealer: &gtk::Revealer, expand: bool, use_transition: bool) {
        if !use_transition {
            let transition_type = revealer.transition_type();
            revealer.set_transition_type(gtk::RevealerTransitionType::None);
            revealer.set_transition_type(transition_type);
        }
        revealer.set_reveal_child(expand);
    }
}
