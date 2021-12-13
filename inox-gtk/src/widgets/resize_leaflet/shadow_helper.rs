use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::SingleSelection;
use gtk::{Application, SignalListItemFactory};
use log::*;


use super::shadow_helper_imp as imp;


glib::wrapper! {
    pub struct ShadowHelper(ObjectSubclass<imp::ShadowHelper>);
}

impl Default for ShadowHelper {
    fn default() -> Self {
        glib::Object::new(&[]).expect("Failed to create ShadowHelper")
    }
}

impl ShadowHelper {
    pub fn new<T: IsA<gtk::Widget>>(widget: &T) -> Self {
        glib::Object::new(&[("widget", widget)]).expect("Failed to create ShadowHelper")
    }

    pub fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let imp = imp::ShadowHelper::from_instance(self);
        imp.snapshot(snapshot)
    }
}
