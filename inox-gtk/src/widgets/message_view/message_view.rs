use crate::core::message::Message;
use crate::core::thread::Thread;
use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use notmuch;

use super::message_view_imp as imp;
use crate::core::Action;

// Wrap imp::MessageView into a usable gtk-rs object
glib::wrapper! {
    pub struct MessageView(ObjectSubclass<imp::MessageView>)
        @extends gtk::Widget;
}

// MessageView implementation itself
impl MessageView {
    pub fn new(message: &notmuch::Message, sender: Sender<Action>) -> Self {
        let view: Self = glib::Object::new(&[]).expect("Failed to create MessageView");
        let imp = imp::MessageView::from_instance(&view);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on MessageView");

        let message = Message::new(message).unwrap();
        imp.message
            .set(message.clone())
            .expect("Failed to set message on MessageView");

        imp.update_compact();

        view
    }

    /**
     * Shows the complete message: headers, body and attachments.
     */
    pub fn expand(&self, include_transitions: bool) {
        // self.set.is_collapsed = false;
        self.update_message_state();

        let imp = imp::MessageView::from_instance(self);
        imp.attachments_button.get().set_sensitive(true);
        // // Needs at least some menu set otherwise it won't be enabled,
        // // also has the side effect of making it sensitive
        // this.email_menubutton.set_menu_model(new GLib.Menu());

        // // Set targets to enable the actions
        // GLib.Variant email_target = email.id.to_variant();
        // this.attachments_button.set_action_target_value(email_target);
        // this.star_button.set_action_target_value(email_target);
        // this.unstar_button.set_action_target_value(email_target);

        imp.show_message_body(include_transitions);
    }

    /**
     * Hides the complete message, just showing the header preview.
     */
    pub fn collapse(&self) {
        // is_collapsed = true;
        self.update_message_state();

        let imp = imp::MessageView::from_instance(self);

        imp.attachments_button.get().set_sensitive(false);
        // imp.message_menubutton.get().set_sensitive(false);

        // // Clear targets to disable the actions
        // this.attachments_button.set_action_target_value(null);
        // this.star_button.set_action_target_value(null);
        // this.unstar_button.set_action_target_value(null);

        // primary_message.hide_message_body();
        // foreach (ConversationMessage attached in this._attached_messages) {
        //     attached.hide_message_body();
        // }
    }

    fn update_message_state(&self) {}
    //     private void update_email_state() {
    //     Gtk.StyleContext style = get_style_context();

    //     if (this.is_unread) {
    //         style.add_class(UNREAD_CLASS);
    //     } else {
    //         style.remove_class(UNREAD_CLASS);
    //     }

    //     if (this.is_starred) {
    //         style.add_class(STARRED_CLASS);
    //         this.star_button.hide();
    //         this.unstar_button.show();
    //     } else {
    //         style.remove_class(STARRED_CLASS);
    //         this.star_button.show();
    //         this.unstar_button.hide();
    //     }

    //     update_email_menu();
    // }

    // private void update_email_menu() {
    //     if (this.email_menubutton.active) {
    //         bool in_base_folder = this.conversation.is_in_base_folder(
    //             this.email.id
    //         );
    //         bool supports_trash = (
    //             in_base_folder &&
    //             Application.Controller.does_folder_support_trash(
    //                 this.conversation.base_folder
    //             )
    //         );
    //         bool supports_delete = (
    //             in_base_folder &&
    //             this.conversation.base_folder is Geary.FolderSupport.Remove
    //         );
    //         bool is_shift_down = false;
    //         var main = get_toplevel() as Application.MainWindow;
    //         if (main != null) {
    //             is_shift_down = main.is_shift_down;

    //             if (!this.shift_handler_installed) {
    //                 this.shift_handler_installed = true;
    //                 main.notify["is-shift-down"].connect(on_shift_changed);
    //             }
    //         }

    //         string[] blacklist = {};
    //         if (this.is_unread) {
    //             blacklist += (
    //                 ConversationListBox.EMAIL_ACTION_GROUP_NAME + "." +
    //                 ConversationListBox.ACTION_MARK_UNREAD
    //             );
    //             blacklist += (
    //                 ConversationListBox.EMAIL_ACTION_GROUP_NAME + "." +
    //                 ConversationListBox.ACTION_MARK_UNREAD_DOWN
    //             );
    //         } else {
    //             blacklist += (
    //                 ConversationListBox.EMAIL_ACTION_GROUP_NAME + "." +
    //                 ConversationListBox.ACTION_MARK_READ
    //             );
    //         }

    //         bool show_trash = !is_shift_down && supports_trash;
    //         bool show_delete = !show_trash && supports_delete;
    //         GLib.Variant email_target = email.id.to_variant();
    //         GLib.Menu new_model = Util.Gtk.construct_menu(
    //             email_menu_template,
    //             (menu, submenu, action, item) => {
    //                 bool accept = true;
    //                 if (submenu == email_menu_trash_section && !show_trash) {
    //                     accept = false;
    //                 }
    //                 if (submenu == email_menu_delete_section && !show_delete) {
    //                     accept = false;
    //                 }
    //                 if (action != null && !(action in blacklist)) {
    //                     item.set_action_and_target_value(
    //                         action, email_target
    //                     );
    //                 }
    //                 return accept;
    //             }
    //         );

    //         this.email_menubutton.popover.bind_model(new_model, null);
    //         this.email_menubutton.popover.grab_focus();
    //     }
    // }
}
