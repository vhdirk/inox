extern crate gtk;
use uibuilder::gtk::BuilderExt;


#[derive(Debug, Clone)]
pub struct UI {
    pub builder: gtk::Builder,
}

impl UI {
    pub fn new() -> UI {
        // The order here is important because some ui file depends on others

        let builder = gtk::Builder::new();

        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/autocomplete.ui")
        //        .expect("Can't load ui file: autocomplete.ui");

        // needed from main_window
        // These are popup menus showed from main_window interface
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/user_menu.ui")
        //        .expect("Can't load ui file: user_menu.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/add_room_menu.ui")
        //        .expect("Can't load ui file: add_room_menu.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/room_menu.ui")
        //        .expect("Can't load ui file: room_menu.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/members.ui")
        //        .expect("Can't load ui file: members.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/markdown_popover.ui")
        //        .expect("Can't load ui file: markdown_popover.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/server_chooser_menu.ui")
        //        .expect("Can't load ui file: server_chooser_menu.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/stickers_popover.ui")
        //        .expect("Can't load ui file: stickers_popover.ui");
        builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/main_window.ui")
               .expect("Can't load ui file: main_window.ui");

        // Depends on main_window
        // These are all dialogs transient for main_window
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/direct_chat.ui")
        //        .expect("Can't load ui file: direct_chat.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/invite.ui")
        //        .expect("Can't load ui file: invite.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/invite_user.ui")
        //        .expect("Can't load ui file: invite_user.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/join_room.ui")
        //        .expect("Can't load ui file: join_room.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/leave_room.ui")
        //        .expect("Can't load ui file: leave_room.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/new_room.ui")
        //        .expect("Can't load ui file: new_room.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/password_dialog.ui")
        //        .expect("Can't load ui file: password_dialog.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/account_settings.ui")
        //        .expect("Can't load ui file: account_settings.ui");
        // builder.add_from_resource("/com/github/vhdirk/Enamel/gtk/msg_src_window.ui")
        //        .expect("Can't load ui file: msg_src_window.ui");

        UI { builder }
    }
}
