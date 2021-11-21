use gtk::prelude::*;

use super::placeholder_pane_imp as imp;

glib::wrapper! {
    pub struct PlaceholderPane(ObjectSubclass<imp::PlaceholderPane>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for PlaceholderPane {
    fn default() -> Self {
        glib::Object::new(&[]).expect("Failed to create PlaceholderPane")
    }
}

impl PlaceholderPane {
    pub fn new(icon_name: &str, title: &str, subtitle: &str) -> Self {
        let pane: Self = glib::Object::new(&[]).expect("Failed to create PlaceholderPane");
        pane.set_property("icon-name", icon_name);
        pane.set_property("title", title);
        pane.set_property("subtitle", subtitle);
        pane
    }
}
