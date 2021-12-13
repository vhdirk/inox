use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::SingleSelection;
use gtk::{Application, SignalListItemFactory};
use log::*;


use super::resize_leaflet_imp as imp;
use super::resize_leaflet_page::ResizeLeafletPage;

glib::wrapper! {
    pub struct ResizeLeaflet(ObjectSubclass<imp::ResizeLeaflet>)
        @extends gtk::Widget, @implements gtk::Orientable, gtk::Buildable, adw::Swipeable;
}

impl ResizeLeaflet {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ResizeLeaflet")
    }

    pub fn children(&self) -> Vec<ResizeLeafletPage> {
        let imp = imp::ResizeLeaflet::from_instance(self);
        imp.children.borrow().clone()
    }

    pub fn visible_child(&self) -> Option<ResizeLeafletPage> {
        let imp = imp::ResizeLeaflet::from_instance(self);
        imp.visible_child.borrow().clone()
    }

    pub fn set_visible_child(&self, child: Option<&ResizeLeafletPage>) {
        let imp = imp::ResizeLeaflet::from_instance(self);
        imp.set_visible_child(child)
    }
}
