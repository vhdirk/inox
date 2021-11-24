use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::sync::Lazy;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::subclass::signal::Signal;
use glib::{ParamFlags, ParamSpec, ParamSpecBoolean, Value};
use gtk::{prelude::*, subclass::prelude::*};
use std::fmt;
pub type BaseRowInstance = super::BaseRow;

#[repr(C)]
pub struct BaseRowClass {
    pub parent_class: gtk::ffi::GtkListBoxRowClass,
    pub expand: fn(&BaseRowInstance),
    pub collapse: fn(&BaseRowInstance),
}

unsafe impl ClassStruct for BaseRowClass {
    type Type = BaseRow;
}

fn expand_default_trampoline(this: &BaseRowInstance) {
    BaseRow::from_instance(this).expand(this)
}

fn collapse_default_trampoline(this: &BaseRowInstance) {
    BaseRow::from_instance(this).collapse(this)
}

pub fn base_row_expand(this: &BaseRowInstance) {
    let klass = this.class();
    (klass.as_ref().expand)(this)
}

pub fn base_row_collapse(this: &BaseRowInstance) {
    let klass = this.class();
    (klass.as_ref().collapse)(this)
}

#[derive(Debug, Default)]
pub struct BaseRow {
    pub expanded: RefCell<bool>,
}

impl BaseRow {
    fn expand(&self, _obj: &BaseRowInstance) {}

    fn collapse(&self, _obj: &BaseRowInstance) {}
}

#[glib::object_subclass]
impl ObjectSubclass for BaseRow {
    const NAME: &'static str = "InoxBaseRow";
    const ABSTRACT: bool = true;
    type Type = BaseRowInstance;
    type ParentType = gtk::ListBoxRow;
    type Class = BaseRowClass;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();

        klass.expand = expand_default_trampoline;
        klass.collapse = collapse_default_trampoline;
    }
}

impl ObjectImpl for BaseRow {
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
impl WidgetImpl for BaseRow {}
impl ListBoxRowImpl for BaseRow {}
