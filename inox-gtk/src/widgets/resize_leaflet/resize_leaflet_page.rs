
use std::cell::{Ref, RefMut};
use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::SingleSelection;
use gtk::{Application, SignalListItemFactory};
use log::*;


use super::resize_leaflet_page_imp as imp;
pub use imp::ResizeLeafletPageData;

glib::wrapper! {
    pub struct ResizeLeafletPage(ObjectSubclass<imp::ResizeLeafletPage>);
}

impl ResizeLeafletPage {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ResizeLeafletPage")
    }

    pub fn child(&self) -> Option<gtk::Widget> {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.child()
    }

    pub fn name(&self) -> Option<String> {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.name()
    }

    pub fn is_navigatable(&self) -> bool {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.is_navigatable()
    }

    pub fn set_name(&self, name: Option<&str>) {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.set_name(name)
    }

    pub fn set_navigatable(&self, navigatable: bool) {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.set_navigatable(navigatable)
    }

    pub fn data(&self) -> Ref<ResizeLeafletPageData> {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.data.borrow()
    }

    pub fn mut_data(&self) -> RefMut<ResizeLeafletPageData> {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.data.borrow_mut()
    }

    pub fn set_data(&self, data: ResizeLeafletPageData) {
        let imp = imp::ResizeLeafletPage::from_instance(self);
        imp.data.replace(data);
    }
}
