use std::cell::RefCell;
use crate::core::Action;
use glib::Sender;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};
use once_cell::unsync::OnceCell;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::{ParamFlags, ParamSpec, Value};
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};

#[derive(Debug, Default)]
pub struct MessageRowBase {
    pub expanded: RefCell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageRowBase {
    const NAME: &'static str = "InoxMessageRowBase";
    const ABSTRACT: bool = true;
    type Type = super::MessageRowBase;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for MessageRowBase {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;

        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpec::new_boolean(
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
}
impl WidgetImpl for MessageRowBase {}
impl ListBoxRowImpl for MessageRowBase {}
