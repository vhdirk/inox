use crate::widgets::ResizeLeaflet;
use adw;
use glib::subclass::prelude::*;
use glib::subclass::signal::Signal;
use glib::{clone, Sender};
use glib::{ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecObject, ParamSpecString, Value};
use gtk;
use gtk::builders::ImageBuilder;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::SignalListItemFactory;
use gtk::{prelude::*, subclass::prelude::*};
use log::*;
use once_cell::sync::{Lazy, OnceCell};
use std::cell::RefCell;
use std::cmp;
use std::fmt;

use super::resize_leaflet_imp;

#[derive(Debug, Clone)]
pub struct ResizeLeafletPageData {
    /* Convenience storage for per-child temporary frequently computed values. */
    pub alloc: gtk::Allocation,
    pub min: gtk::Requisition,
    pub nat: gtk::Requisition,
    pub visible: bool,
    pub last_focus: Option<gtk::Widget>,
}

impl Default for ResizeLeafletPageData {
    fn default() -> Self {
        Self {
            alloc: gtk::Allocation::new(0, 0, 0, 0),
            min: gtk::Requisition::default(),
            nat: gtk::Requisition::default(),
            visible: false,
            last_focus: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct ResizeLeafletPage {
    pub widget: RefCell<Option<gtk::Widget>>,
    pub name: RefCell<Option<String>>,
    pub navigatable: RefCell<bool>,

    pub data: RefCell<ResizeLeafletPageData>,
}

#[glib::object_subclass]
impl ObjectSubclass for ResizeLeafletPage {
    const NAME: &'static str = "InoxResizeLeafletPage";
    type Type = super::ResizeLeafletPage;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for ResizeLeafletPage {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecObject::new(
                    "child",
                    "Child",
                    "The child of the page",
                    gtk::Widget::static_type(),
                    ParamFlags::READWRITE,
                ),
                ParamSpecString::new(
                    "name",
                    "Name",
                    "The name of the child page",
                    None,
                    ParamFlags::READWRITE,
                ),
                ParamSpecBoolean::new(
                    "navigatable",
                    "Navigatable",
                    "Whether the child can be navigated to",
                    true,
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "child" => {
                self.widget.replace(value.get().unwrap());
            }
            "name" => {
                self.set_name(value.get().unwrap());
            }
            "navigatable" => {
                self.set_navigatable(value.get().unwrap());
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "child" => self.widget.borrow().to_value(),
            "name" => self.name.borrow().to_value(),
            "navigatable" => self.navigatable.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {}
}
impl WidgetImpl for ResizeLeafletPage {}

impl ResizeLeafletPage {
    pub fn set_name(&self, name: Option<&str>) {
        let inst = self.instance();
        let strname = name.map(|s| s.to_string());

        if let Some(widget) = self.widget.borrow().as_ref() {
            if let Some(parent) = widget.parent() {
                if let Some(leaflet) = parent.downcast_ref::<ResizeLeaflet>() {
                    for page in leaflet.children() {
                        if inst.eq(&page) {
                            continue;
                        }

                        if strname.eq(&page.name()) {
                            warn!("Duplicate child name in ResizeLeaflet: {:?}", name);
                        }
                    }
                }
            }
        }

        if strname.eq(&self.name.borrow()) {
            return;
        }

        self.name.replace(strname);

        inst.notify_by_pspec(&Self::properties()[1]);

        if let Some(widget) = self.widget.borrow().as_ref() {
            if let Some(parent) = widget.parent() {
                if let Some(leaflet) = parent.downcast_ref::<ResizeLeaflet>() {
                    if let Some(visible_child) = leaflet.visible_child() {
                        if visible_child.eq(&inst) {
                            leaflet.notify_by_pspec(
                                &resize_leaflet_imp::ResizeLeaflet::properties()[3],
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn set_navigatable(&self, navigatable: bool) {
        let inst = self.instance();

        if *self.navigatable.borrow() == navigatable {
            return;
        }

        self.navigatable.replace(navigatable);

        if let Some(widget) = self.widget.borrow().as_ref() {
            if let Some(parent) = widget.parent() {
                if let Some(leaflet) = parent.downcast_ref::<ResizeLeaflet>() {
                    if let Some(visible_child) = leaflet.visible_child() {
                        if visible_child.eq(&inst) {
                            leaflet.set_visible_child(None);
                        }
                    }
                }
            }
        }
        inst.notify_by_pspec(&Self::properties()[2]);
    }

    pub fn child(&self) -> Option<gtk::Widget> {
        self.widget.borrow().clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.borrow().clone()
    }

    pub fn is_navigatable(&self) -> bool {
        *self.navigatable.borrow()
    }
}
