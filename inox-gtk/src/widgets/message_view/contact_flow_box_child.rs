use glib;
use gtk;
use super::contact_flow_box_child_imp as imp;

// Wrap imp::ContactFlowBoxChild into a usable gtk-rs object
glib::wrapper! {
    pub struct ContactFlowBoxChild(ObjectSubclass<imp::ContactFlowBoxChild>)
        @extends gtk::FlowBoxChild, gtk::Widget;
}


    // // Widget used to display sender/recipient email addresses in
    // // message header Gtk.FlowBox instances.
    // private class ContactFlowBoxChild : Gtk.FlowBoxChild {


    //     private const string PRIMARY_CLASS = "geary-primary";


    //     public enum Type { FROM, OTHER; }


    //     public Type address_type { get; private set; }

    //     public Application.Contact contact { get; private set; }

    //     public Geary.RFC822.MailboxAddress displayed { get; private set; }
    //     public Geary.RFC822.MailboxAddress source { get; private set; }

    //     private string search_value;

    //     private Gtk.Bin container;


    //     public ContactFlowBoxChild(Application.Contact contact,
    //                                Geary.RFC822.MailboxAddress source,
    //                                Type address_type = Type.OTHER) {
    //         this.contact = contact;
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

    //         this.contact.changed.connect(on_contact_changed);
    //         update();
    //     }

    //     public override void destroy() {
    //         this.contact.changed.disconnect(on_contact_changed);
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

    //     private void update() {
    //         // We use two GTK.Label instances here when address has
    //         // distinct parts so we can dim the secondary part, if
    //         // any. Ideally, it would be just one label instance in
    //         // both cases, but we can't yet include CSS classes in
    //         // Pango markup. See Bug 766763.

    //         Gtk.Grid address_parts = new Gtk.Grid();

    //         bool is_spoofed = this.source.is_spoofed();
    //         if (is_spoofed) {
    //             Gtk.Image spoof_img = new Gtk.Image.from_icon_name(
    //                 "dialog-warning-symbolic", Gtk.IconSize.SMALL_TOOLBAR
    //             );
    //             this.set_tooltip_text(
    //                 _("This email address may have been forged")
    //             );
    //             address_parts.add(spoof_img);
    //             get_style_context().add_class(SPOOF_CLASS);
    //         }

    //         Gtk.Label primary = new Gtk.Label(null);
    //         primary.ellipsize = Pango.EllipsizeMode.END;
    //         primary.set_halign(Gtk.Align.START);
    //         primary.get_style_context().add_class(PRIMARY_CLASS);
    //         if (this.address_type == Type.FROM) {
    //             primary.get_style_context().add_class(FROM_CLASS);
    //         }
    //         address_parts.add(primary);

    //         string display_address = this.source.to_address_display("", "");

    //         if (is_spoofed || this.contact.display_name_is_email) {
    //             // Don't display the name to avoid duplication and/or
    //             // reduce the chance of the user of being tricked by
    //             // malware.
    //             primary.set_text(display_address);
    //             // Use the source as the displayed address so that the
    //             // contact popover uses the spoofed mailbox and
    //             // displays it as being spoofed.
    //             this.displayed = this.source;
    //         } else if (this.contact.is_trusted) {
    //             // The contact's name can be trusted, so no need to
    //             // display the email address
    //             primary.set_text(this.contact.display_name);
    //             this.displayed = new Geary.RFC822.MailboxAddress(
    //                 this.contact.display_name, this.source.address
    //             );
    //             this.tooltip_text = this.source.address;
    //         } else {
    //             // Display both the display name and the email address
    //             // so that the user has the full information at hand
    //             primary.set_text(this.contact.display_name);
    //             this.displayed = new Geary.RFC822.MailboxAddress(
    //                 this.contact.display_name, this.source.address
    //             );

    //             Gtk.Label secondary = new Gtk.Label(null);
    //             secondary.ellipsize = Pango.EllipsizeMode.END;
    //             secondary.set_halign(Gtk.Align.START);
    //             secondary.get_style_context().add_class(Gtk.STYLE_CLASS_DIM_LABEL);
    //             secondary.set_text(display_address);
    //             address_parts.add(secondary);
    //         }

    //         Gtk.Widget? existing_ui = this.container.get_child();
    //         if (existing_ui != null) {
    //             this.container.remove(existing_ui);
    //         }

    //         this.container.add(address_parts);
    //         show_all();
    //     }

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

    // }

    // /**