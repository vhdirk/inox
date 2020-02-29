use gtk;
use gtk::prelude::*;

// Totally copied it from fractal.
// https://gitlab.gnome.org/danigm/fractal/blob/503e311e22b9d7540089d735b92af8e8f93560c5/fractal-gtk/src/app.rs#L1883-1912
/// Given a `window` create and attach an `gtk::AboutDialog` to it.
pub fn about_dialog(window: &gtk::ApplicationWindow) {
    // Feel free to add yourself if you contribured.
    let authors = &["Dirk Van Haerenborgh"];

    let dialog = gtk::AboutDialog::new();
    // Waiting for a logo.
    dialog.set_logo_icon_name(Some("email"));
    dialog.set_comments(Some("Email with Notmuch Rust."));
    dialog.set_copyright(Some("© 2018 Dirk Van Haerenborgh"));
    dialog.set_license_type(gtk::License::Gpl30);
    dialog.set_modal(true);
    // TODO: make it show it fetches the commit hash from which it was built
    // and the version number is kept in sync automaticly
    dialog.set_version(Some("0.0.1"));
    dialog.set_program_name("Enamel");
    // TODO: Need a wiki page first.
    dialog.set_website(Some("https://github.com/vhdirk/enamel"));
    // dialog.set_website_label("Learn more about Enamel");
    dialog.set_transient_for(Some(window));

    // dialog.set_artists(&["Dirk Van Haerenborgh"]);
    dialog.set_authors(authors);

    dialog.connect_response(|dlg, _| dlg.close());

    dialog.show();
}
