use crate::core::Action;
use crate::widgets::MessageRow;
use crate::widgets::expander_row::{ExpanderRowExt, ExpanderRow};
use glib::subclass::prelude::*;
use glib::{clone, Sender};
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;

#[derive(Debug)]
pub struct MessageList {
    pub list_box: gtk::ListBox,
    pub rows: RefCell<Vec<ExpanderRow>>,
    pub thread: OnceCell<notmuch::Thread>,
    pub sender: OnceCell<Sender<Action>>,

    pub row_activated_handler_id: RefCell<Option<glib::SignalHandlerId>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MessageList {
    const NAME: &'static str = "InoxMessageList";
    type Type = super::MessageList;
    type ParentType = gtk::Widget;

    fn new() -> Self {
        Self {
            list_box: gtk::ListBox::new(),
            rows: RefCell::new(vec![]),
            thread: OnceCell::new(),
            sender: OnceCell::new(),
            row_activated_handler_id: RefCell::new(None),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for MessageList {
    fn constructed(&self, obj: &Self::Type) {
        self.list_box.set_parent(obj);
        self.parent_constructed(obj);

        self.row_activated_handler_id.replace(Some(self.list_box
            .connect_row_activated(clone!(@weak obj => move |list_box, row| {
                let this = MessageList::from_instance(&obj);
                this.on_row_activated(list_box, row)
            }))));
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(id) = self.row_activated_handler_id.borrow_mut().take() {
            self.list_box.disconnect(id);
        }

        let mut rows = self.rows.borrow_mut();
        for row in rows.iter() {
            row.unparent();
        }
        rows.clear();
        self.list_box.unparent();
    }
}
impl WidgetImpl for MessageList {}

impl MessageList {
    pub fn on_row_activated(&self, list_box: &gtk::ListBox, row: &gtk::ListBoxRow) {
        let expander_row = row.clone().downcast::<ExpanderRow>();
        if let Ok(row) = expander_row {
            // Allow non-last rows to be expanded/collapsed, but also let
            // the last row to be expanded since appended sent emails will
            // be appended last. Finally, don't let rows with active
            // composers be collapsed.
            if row.property::<bool>("expanded") {
                if list_box.row_at_index(row.index() + 1).is_some() {
                    row.collapse();
                }
            } else {
                row.expand();
            }
        }
    }

    pub fn init(&self) {

        let inst = self.instance();

        inst.style_context().add_class("content");
        inst.style_context().add_class("background");
        inst.style_context().add_class("messages-list");

        if let Some(thread) = self.thread.get().as_ref() {
           let messages = thread.messages();
            for message in messages {
                self.add_message(&message);
            }
        }

    }

    pub fn add_message(&self, message: &notmuch::Message) {
        let message_row = MessageRow::new(message, self.sender.get().unwrap().clone());
        self.list_box.append(&message_row);
        self.rows
            .borrow_mut()
            .push(message_row.upcast::<ExpanderRow>());
    }


}
