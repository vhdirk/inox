use crate::widgets::message_view::ContactFlowBoxChild;
use std::cell::RefCell;

use crate::core::Action;
use crate::core::Message;
use crate::widgets::web_view::MessageWebView;
use chrono_humanize::HumanTime;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::{clone, Sender};
use gmime;
use gmime::traits::MessageExt;
use gmime::{InternetAddressExt, InternetAddressListExt};

use crate::widgets::web_view::web_view_imp;
use gtk::{self, prelude::*, subclass::prelude::*, CompositeTemplate};
use once_cell::unsync::OnceCell;

const EMPTY_FROM_LABEL: &str = "No sender";

// TODO: make configurable
const MARK_READ_TIMEOUT_MSEC: u64 = 250;

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

    pub read_timeout_source_id: RefCell<Option<glib::SourceId>>,
    pub address_flowbox_children: RefCell<Vec<gtk::FlowBoxChild>>,
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

            read_timeout_source_id: RefCell::new(None),
            address_flowbox_children: RefCell::new(vec![]),
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

        self.body_container.get().set_has_tooltip(true); // Used to show link URLs

        // TODO: should only be launched when email is actually opened
        dbg!("Starting unread timeout");
        // mark the message as read if this view is open for `n` seconds
        self.read_timeout_source_id
            .replace(Some(glib::source::timeout_add_local_once(
                std::time::Duration::from_millis(MARK_READ_TIMEOUT_MSEC),
                clone!(@weak obj => move || {
                    let this = Self::from_instance(&obj);
                    this.read_timeout_source_id.replace(None);
                    this.mark_read();
                }),
            )));
    }

    fn dispose(&self, obj: &Self::Type) {
        // TODO: not sure why we need to unparent these manually?
        self.actions.get().unparent();
        self.message_container.get().unparent();

        let mut address_flowbox_children = self.address_flowbox_children.borrow_mut();
        for child in address_flowbox_children.iter() {
            child.unparent();
        }
        address_flowbox_children.clear();

        if let Some(view) = self.web_view.get() {
            view.unparent();
        }

        if let Some(id) = self.read_timeout_source_id.take() {
            id.remove();
        }
    }
}
impl WidgetImpl for MessageView {}

impl MessageView {
    pub fn init(&self) {}

    pub fn update_display(&self) {
        self.compact_body
            .get()
            .set_text(&self.format_body_compact());
        self.compact_from
            .get()
            .set_text(&self.format_originator_compact());
        self.compact_date
            .get()
            .set_text(&self.format_date_compact());

        self.subject.get().set_text(&self.format_subject());
        self.date.get().set_text(&self.format_date());
    }

    pub fn fill_originator_addresses(&self) {
        let msg = self.message.get().unwrap();

        // Show any From header addresses
        let from = msg.from();

        dbg!(&from);

        if from.is_some() && from.as_ref().unwrap().length() > 0 {
            let from = from.as_ref().unwrap();
            for i in 0..from.length() {
                let address = from.address(i).unwrap();
                let child = ContactFlowBoxChild::new(
                    &address,
                    gmime::AddressType::From,
                    self.sender.get().unwrap().clone(),
                );
                child.show();
                //         this.searchable_addresses.add(child);
                self.from.get().set_visible(true);
                self.from.get().insert(&child, i);
                self.address_flowbox_children.borrow_mut().push(child.upcast::<gtk::FlowBoxChild>());
            }
        } else {
            let label = gtk::Label::new(Some(EMPTY_FROM_LABEL));
            let child = gtk::FlowBoxChild::new();
            label.set_parent(&child);
            child.set_halign(gtk::Align::Start);
            child.show();
            //child.set_parent(&self.from.get());
            self.from.set_visible(true);
            self.from.get().insert(&child, -1);
            self.address_flowbox_children.borrow_mut().push(child);
        }

        let sender = msg.sender();
        // Show the Sender header addresses if present, but only if
        // not already in the From header.
        if sender.is_some() && sender.as_ref().unwrap().length() > 0 {
            let sender = sender.as_ref().unwrap();
            for i in 0..sender.length() {
                let address = sender.address(i).unwrap();
                dbg!("sender ", &address);
                if from.is_some()
                    && from.as_ref().unwrap().length() > 0
                    && from.as_ref().unwrap().contains(&address)
                {
                    continue;
                }
                let child = ContactFlowBoxChild::new(
                    &address,
                    gmime::AddressType::Sender,
                    self.sender.get().unwrap().clone(),
                );
                child.show();
                //         self.searchable_addresses.add(child);
                self.sender_header.get().show();
                self.sender_header.get().set_visible(true);
                self.sender_address.get().insert(&child, i);
                self.address_flowbox_children.borrow_mut().push(child.upcast::<gtk::FlowBoxChild>());
            }
        }

        let reply_to = msg.reply_to();
        // Show any Reply-To header addresses if present, but only if
        // each is not already in the From header.
        if reply_to.is_some() && reply_to.as_ref().unwrap().length() > 0 {
            let reply_to = reply_to.as_ref().unwrap();
            for i in 0..reply_to.length() {
                let address = reply_to.address(i).unwrap();
                if from.is_some()
                    && from.as_ref().unwrap().length() > 0
                    && from.as_ref().unwrap().contains(&address)
                {
                    continue;
                }
                let child = ContactFlowBoxChild::new(
                    &address,
                    gmime::AddressType::ReplyTo,
                    self.sender.get().unwrap().clone(),
                );
                child.show();
                //         self.searchable_addresses.add(child);
                self.reply_to_header.get().show();
                self.reply_to_addresses.get().set_visible(true);

                self.reply_to_addresses.get().insert(&child, i);
                self.address_flowbox_children.borrow_mut().push(child.upcast::<gtk::FlowBoxChild>());
            }
        }
    }

    pub fn fill_addresses(&self) {}

    pub fn format_subject(&self) -> String {
        let msg = self.message.get().unwrap();
        msg.subject()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "".to_string())
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

    pub fn format_date(&self) -> String {
        let msg = self.message.get().unwrap();
        msg.date().to_rfc2822()
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

        self.connect_web_view_signals();
    }

    pub fn hide_message_body(&self) {
        self.compact_revealer.get().set_reveal_child(true);
        self.header_revealer.get().set_reveal_child(false);
        self.body_revealer.get().set_reveal_child(false);
    }

    pub fn set_expanded(&self, expanded: bool) {
        self.date.get().set_visible(expanded);
        self.subject.get().set_visible(expanded);

        self.attachments_button.get().set_sensitive(expanded);
        // self.message_menubutton.get().set_sensitive(expanded);

        self.compact_from.get().set_visible(!expanded);
        self.compact_date.get().set_visible(!expanded);
        self.compact_body.get().set_visible(!expanded);
    }

    pub fn initialize_web_view(&self) {
        dbg!("initialize_web_view {:?}", self.web_view.get());

        let web_view = MessageWebView::new(self.sender.get().unwrap().clone());
        // web_view.set_parent(&self.body_container.get());
        self.body_container.get().show();
        self.body_container.get().set_vexpand(true);
        self.body_container.get().set_hexpand(true);
        self.body_container.get().attach(&web_view, 0, 0, 1, 1);

        self.web_view.set(web_view).unwrap();
    }

    pub fn connect_web_view_signals(&self) {
        let inst = self.instance();

        self.web_view.get().as_ref().unwrap().connect_local_id(
            web_view_imp::WebView::signals()[0].signal_id(),
            None,
            false,
            clone!(@weak inst => @default-return None, move |_| {
                dbg!("on content-loaded");
                let this = Self::from_instance(&inst);
                this.on_content_loaded();
                None
            }),
        );

        // let cinst = inst.clone();
        // self.web_view.get().connect_flag_remote_images(move || {
        //     let this = Self::from_instance(cinst);
        //     this.on_flag_remote_images();
        // });

        // let cinst = inst.clone();
        // self.web_view
        //     .get()
        //     .connect_internal_link_activated(move |y| {
        //         let this = Self::from_instance(cinst);
        //         this.internal_link_activated(y);
        //     });

        // let cinst = inst.clone();
        // self.web_view
        //     .get()
        //     .connect_internal_resource_loaded(move |id| {
        //         let this = Self::from_instance(cinst);
        //         this.on_resource_loaded(id);
        //     });

        // let cinst = inst.clone();
        // self.web_view.get().connect_save_image(move |id| {
        //     let this = Self::from_instance(cinst);
        //     this.on_save_image(id);
        // });

        // let cinst = inst.clone();
        // self.web_view.get().connect_selection_changed((has_selection) => {
        //         this.body_selection_message = has_selection ? view : null;
        //         body_selection_changed(has_selection);
        //     });
    }

    pub fn update_displayed_attachments(&self) {
        // bool has_attachments = !this.displayed_attachments.is_empty;
        // this.attachments_button.set_visible(has_attachments);
        // var main = get_toplevel() as Application.MainWindow;

        // if (has_attachments && main != null) {
        //     this.attachments_pane = new Components.AttachmentPane(
        //         false, main.attachments
        //     );
        //     this.primary_message.body_container.add(this.attachments_pane);

        //     foreach (var attachment in this.displayed_attachments) {
        //         this.attachments_pane.add_attachment(
        //             attachment, this.load_cancellable
        //         );
        //     }
        // }
    }

    pub fn on_content_loaded(&self) {
        self.fill_originator_addresses();
        // bool all_loaded = true;
        // foreach (ConversationMessage message in this) {
        //     if (!message.is_content_loaded) {
        //         all_loaded = false;
        //         break;
        //     }
        // }
        // if (all_loaded && this.message_body_state != COMPLETED) {
        //     this.message_body_state = COMPLETED;
        //     this.message_bodies_loaded_lock.blind_notify();

        // Update attachments once the web views have finished
        // loading, since we want to know if any attachments
        // marked as being inline were actually not displayed
        // inline, and hence need to be displayed as if they were
        // attachments.
        self.update_displayed_attachments();
        // }
    }

    /**
     * Starts loading the message body in the HTML view.
     */
    pub fn load_message_body(&self) {
        // throws GLib.Error {
        // if (load_cancelled.is_cancelled()) {
        //     throw new GLib.IOError.CANCELLED("Conversation load cancelled");
        // }

        if self.web_view.get().is_none() {
            self.initialize_web_view()
        }

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

    pub fn mark_read(&self) {
        // if let Some(msg) = self.message.get() {
        //     if msg.is_read() {
        //         return;
        //     }

        //     let mut msg = msg.clone();
        //     msg.set_read(true);
        //     self.sender.get().unwrap().send(Message::UpdateMessage(msg)).unwrap();
        // }
    }
}
