
use inox_core::models::Contact;
use crate::core::internet_address::InternetAddressAux;
use gmime::InternetAddressExt;
use glib::Sender;
use crate::core::Action;
use once_cell::sync::OnceCell;
use gtk;
use gtk::prelude::*;
use glib::subclass::prelude::*;
use gtk::subclass::prelude::*;
use pango;
use gmime;
use log::*;


#[derive(Debug, Default)]
pub struct ContactFlowBoxChild {
    pub sender: OnceCell<Sender<Action>>,
    pub contact: OnceCell<Contact>,
    pub address_type: OnceCell<gmime::AddressType>,
    pub displayed: String,
    pub source: String,

    pub address_parts: gtk::Grid
}


#[glib::object_subclass]
impl ObjectSubclass for ContactFlowBoxChild {
    const NAME: &'static str = "InoxContactFlowBoxChild";
    type Type = super::ContactFlowBoxChild;
    type ParentType = gtk::FlowBoxChild;
}



impl ObjectImpl for ContactFlowBoxChild {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        self.address_parts.set_parent(obj);
        obj.set_halign(gtk::Align::Start);

    }

    fn dispose(&self, obj: &Self::Type) {
        self.address_parts.unparent();
    }
}
impl WidgetImpl for ContactFlowBoxChild {}

impl FlowBoxChildImpl for ContactFlowBoxChild {}

impl ContactFlowBoxChild {

    // // Widget used to display sender/recipient email addresses in
    // // message header Gtk.FlowBox instances.
    // private class ContactFlowBoxChild : Gtk.FlowBoxChild {


    //     private const string PRIMARY_CLASS = "geary-primary";


    //     public enum Type { FROM, OTHER; }


    //     public Type address_type { get; private set; }

    //     public Application.address address { get; private set; }

    //     public Geary.RFC822.MailboxAddress displayed { get; private set; }
    //     public Geary.RFC822.MailboxAddress source { get; private set; }

    //     private string search_value;

    //     private Gtk.Bin container;


    //     public ContactFlowBoxChild(Application.address address,
    //                                Geary.RFC822.MailboxAddress source,
    //                                Type address_type = Type.OTHER) {
    //         this.address = address;
    //         this.source = source;
    //         this.address_type = address_type;
    //         this.search_value = source.to_searchable_string().casefold();

    //         // Update prelight state when mouse-overed.
    //         Gtk.EventBox events = new Gtk.EventBox();
    //         events.add_events(
    //             Gdk.EventMask.ENTER_NOTIFY_MASK |
    //             Gdk.EventMask.LEAVE_NOTIFY_MASK
    //         );
    //         events.set_visible_window(false);
    //         events.enter_notify_event.connect(on_prelight_in_event);
    //         events.leave_notify_event.connect(on_prelight_out_event);

    //         add(events);
    //         this.container = events;
    //         set_halign(Gtk.Align.START);

    //         this.address.changed.connect(on_contact_changed);
    //         update();
    //     }

    //     public override void destroy() {
    //         this.address.changed.disconnect(on_contact_changed);
    //         base.destroy();
    //     }

    //     public bool highlight_search_term(string term) {
    //         bool found = term in this.search_value;
    //         if (found) {
    //             get_style_context().add_class(MATCH_CLASS);
    //         } else {
    //             get_style_context().remove_class(MATCH_CLASS);
    //         }
    //         return found;
    //     }

    //     public void unmark_search_terms() {
    //         get_style_context().remove_class(MATCH_CLASS);
    //     }

    pub fn update(&self) {
        let inst = self.instance();
        // We use two GTK.Label instances here when address has
        // distinct parts so we can dim the secondary part, if
        // any. Ideally, it would be just one label instance in
        // both cases, but we can't yet include CSS classes in
        // Pango markup. See Bug 766763.
        let contact = self.contact.get().expect("contact should be set");
        let address_type = self.address_type.get().expect("address_type should be set");

        // if address.is_spoofed() {
        //     let spoof_img = gtk::Image::from_icon_name(
        //         Some("dialog-warning-symbolic")
        //     );
        //     inst.set_tooltip_text(
        //         Some("This email address may have been forged")
        //     );
        //     self.address_parts.attach(&spoof_img, 0, 0, 1, 1);

        //     // get_style_context().add_class(SPOOF_CLASS);
        // }

        let primary = gtk::Label::new(None);
        primary.set_ellipsize(pango::EllipsizeMode::End);
        primary.set_xalign(0.0);
        // primary.get_style_context().add_class(PRIMARY_CLASS);
        if (gmime::AddressType::From.eq(address_type)) {
            // primary.style_context().add_class(FROM_CLASS);
        }
        self.address_parts.attach(&primary, 1, 0, 1, 1);

        // let display_address = this.source.to_address_display("", "");

    //         if (is_spoofed || this.address.display_name_is_email) {
    //             // Don't display the name to avoid duplication and/or
    //             // reduce the chance of the user of being tricked by
    //             // malware.
    //             primary.set_text(display_address);
    //             // Use the source as the displayed address so that the
    //             // address popover uses the spoofed mailbox and
    //             // displays it as being spoofed.
    //             this.displayed = this.source;
    //         } else if (this.address.is_trusted) {
    //             // The address's name can be trusted, so no need to
    //             // display the email address
    //             primary.set_text(this.address.display_name);
    //             this.displayed = new Geary.RFC822.MailboxAddress(
    //                 this.address.display_name, this.source.address
    //             );
    //             this.tooltip_text = this.source.address;
    //         } else {
    //             // Display both the display name and the email address
    //             // so that the user has the full information at hand

        debug!("contact name: {:?}", &contact.name.as_ref().unwrap());
        primary.set_text(&contact.name.as_ref().unwrap());
    //             this.displayed = new Geary.RFC822.MailboxAddress(
    //                 this.address.display_name, this.source.address
    //             );

        let secondary = gtk::Label::new(None);
        secondary.set_ellipsize(pango::EllipsizeMode::End);
        secondary.set_xalign(0.0);
        // secondary.style_context().add_class(gtk::STYLE_CLASS_DIM_LABEL);
        // secondary.set_text(display_address);
        self.address_parts.attach(&secondary, 2, 0, 1, 1);

    //         }

    //         Gtk.Widget? existing_ui = this.container.get_child();
    //         if (existing_ui != null) {
    //             this.container.remove(existing_ui);
    //         }

        self.address_parts.show();
    }

    //     private void on_contact_changed() {
    //         update();
    //     }

    //     private bool on_prelight_in_event(Gdk.Event event) {
    //         set_state_flags(Gtk.StateFlags.PRELIGHT, false);
    //         return Gdk.EVENT_STOP;
    //     }

    //     private bool on_prelight_out_event(Gdk.Event event) {
    //         unset_state_flags(Gtk.StateFlags.PRELIGHT);
    //         return Gdk.EVENT_STOP;
    //     }

}