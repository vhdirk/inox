use crate::core::Action;
use crate::widgets::MessageView;
use once_cell::sync::OnceCell;
use std::cell::RefCell;

use glib::subclass::signal::Signal;
use glib::{self, prelude::*, subclass::prelude::*};
use glib::{ParamFlags, ParamSpec, ParamSpecBoolean, Sender, Value};
use gtk::{self, prelude::*, subclass::prelude::*};

use crate::widgets::expander_row::{ExpanderRow, ExpanderRowImpl};

#[derive(Debug)]
pub struct MessageRow {
    pub sender: OnceCell<Sender<Action>>,
    pub view: OnceCell<MessageView>,
    pub pinned: RefCell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageRow {
    const NAME: &'static str = "InoxMessageRow";
    type Type = super::MessageRow;
    type ParentType = ExpanderRow;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn new() -> Self {
        Self {
            sender: OnceCell::new(),
            view: OnceCell::new(),
            pinned: RefCell::new(false),
        }
    }
}

impl ObjectImpl for MessageRow {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_vexpand(true);
        obj.set_hexpand(true);

        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(view) = self.view.get() {
            view.unparent();
        }
    }

    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;

        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecBoolean::new(
                // Name
                "pinned",
                // Nickname
                "pinned",
                // Short description
                "Expanded to show search matches",
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
            "pinned" => {
                self.pinned.replace(value.get().unwrap());
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "pinned" => self.pinned.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for MessageRow {}
impl ListBoxRowImpl for MessageRow {}
impl ExpanderRowImpl for MessageRow {

    fn expand(&self, obj: &ExpanderRow) {
        obj.set_property("expanded", true);

        self.update_row_expansion(obj);

        if let Some(view) = self.view.get() {
            view.load_body();
        }
            // throws GLib.Error {
            // update_row_expansion();
            // if (this.view.message_body_state == NOT_STARTED) {
            //     yield this.view.load_body();
            //     email_loaded(this.view.email);
            // }
    }

    fn collapse(&self, obj: &ExpanderRow) {
        obj.set_property("expanded", false);
        obj.set_property("pinned", false);

        self.update_row_expansion(obj);
    }
}

impl MessageRow {

    pub fn set_view(&self, view: &MessageView) {
        self.instance().set_child(Some(view));
        self.view
            .set(view.clone())
            .expect("Failed to set view on MessageRow");
    }

    pub fn update_row_expansion(&self, obj: &ExpanderRow) {
        let expanded = obj.property::<bool>("expanded");
        let pinned = obj.property::<bool>("pinned");

        if expanded || pinned {
            self.view.get().unwrap().expand(true);
        } else {
            self.view.get().unwrap().collapse();
        }
    }
}
