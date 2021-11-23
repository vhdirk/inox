use std::cell::RefCell;
use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::unsync::OnceCell;
use once_cell::sync::Lazy;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::{ParamFlags, ParamSpec, ParamSpecBoolean, Value};
use glib::subclass::signal::Signal;
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};

pub type BaseRowInstance = super::BaseRow;

#[repr(C)]
pub struct BaseRowClass {
    pub parent_class: gtk::ffi::GtkListBoxRowClass,
    // If these functions are meant to be called from C, you need to make these functions
    // `extern "C"` & use FFI-safe types (usually raw pointers).
    pub expand: Option<unsafe fn(&BaseRowInstance)>,
    pub collapse: Option<unsafe fn(&BaseRowInstance)>,
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

pub unsafe fn base_row_expand(
    this: &BaseRowInstance,
) {
    let klass = &*(this.class() as *const _ as *const BaseRowClass);
    (klass.expand.unwrap())(this)
}

pub unsafe fn base_row_collapse(
    this: &BaseRowInstance,
) {
    let klass = &*(this.class() as *const _ as *const BaseRowClass);
    (klass.collapse.unwrap())(this)
}


#[derive(Debug, Default)]
pub struct BaseRow {
    pub expanded: RefCell<bool>,
}

impl BaseRow {
    fn expand(&self, obj: &BaseRowInstance) {

    }

    fn collapse(&self, obj: &BaseRowInstance) {

    }
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

        klass.expand = Some(expand_default_trampoline);
        klass.collapse = Some(collapse_default_trampoline);
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
