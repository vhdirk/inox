use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::sync::Lazy;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

use glib::subclass::prelude::*;
use glib::subclass::signal::Signal;
use glib::{ParamFlags, ParamSpec, ParamSpecBoolean, Value};
use gtk::{prelude::*, subclass::prelude::*};
use std::fmt;

pub type ExpanderRowInstance = super::ExpanderRow;

#[repr(C)]
pub struct ExpanderRowClass {
    pub parent_class: gtk::ffi::GtkListBoxRowClass,
    pub expand: fn(&ExpanderRowInstance),
    pub collapse: fn(&ExpanderRowInstance),
}

unsafe impl ClassStruct for ExpanderRowClass {
    type Type = ExpanderRow;
}

fn expand_default_trampoline(this: &ExpanderRowInstance) {
    ExpanderRow::from_instance(this).expand(this)
}

fn collapse_default_trampoline(this: &ExpanderRowInstance) {
    ExpanderRow::from_instance(this).collapse(this)
}

pub fn base_row_expand(this: &ExpanderRowInstance) {
    let klass = this.class();
    (klass.as_ref().expand)(this)
}

pub fn base_row_collapse(this: &ExpanderRowInstance) {
    let klass = this.class();
    (klass.as_ref().collapse)(this)
}

#[derive(Debug, Default)]
pub struct ExpanderRow {
    pub expanded: RefCell<bool>,
}

impl ExpanderRow {
    fn expand(&self, _obj: &ExpanderRowInstance) {}

    fn collapse(&self, _obj: &ExpanderRowInstance) {}
}

#[glib::object_subclass]
impl ObjectSubclass for ExpanderRow {
    const NAME: &'static str = "InoxExpanderRow";
    const ABSTRACT: bool = true;
    type Type = ExpanderRowInstance;
    type ParentType = gtk::ListBoxRow;
    type Class = ExpanderRowClass;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();

        klass.expand = expand_default_trampoline;
        klass.collapse = collapse_default_trampoline;
    }
}

impl ObjectImpl for ExpanderRow {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecBoolean::new(
                // Name
                "expanded",
                // Nickname
                "expanded",
                // Short description
                "Is this row expanded",
                // Default value
                false,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "expanded" => {
                self.expanded.replace(value.get().unwrap());
                self.update_css_class();
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "expanded" => self.expanded.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                // Signal name
                "expanded",
                // Types of the values which will be sent to the signal handler
                &[bool::static_type().into()],
                // Type of the value the signal handler sends back
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for ExpanderRow {}
impl ListBoxRowImpl for ExpanderRow {}

impl ExpanderRow {

    pub fn update_css_class(&self) {
        let inst = self.instance();
        if *self.expanded.borrow() {
            inst.style_context().add_class("inox-expanded");
        } else {
            inst.style_context().remove_class("inox-expanded");
        }
        self.update_previous_sibling_css_class();
    }

    pub fn update_previous_sibling_css_class(&self) {
        let inst = self.instance();

        if let Some(previous_sibling) = inst.prev_sibling() {
            if *self.expanded.borrow() {
                previous_sibling.style_context().add_class("inox-expanded-previous-sibling");
            } else {
                previous_sibling.style_context().remove_class("inox-expanded-previous-sibling");
            }
        }
    }
}
