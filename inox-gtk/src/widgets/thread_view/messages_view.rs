use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::{Application, SignalListItemFactory, SingleSelection};
use std::cell::RefCell;

use notmuch;

use super::message_row::MessageRow;
use super::message_row_base::MessageRowBase;
use crate::app::Action;
use inox_core::database::Thread;

/**
 * MessagesView
 *
 * Shows a list of row items, which can be messages, loading animations or composer
 */
mod imp {
    use super::MessageRowBase;
    use crate::app::Action;
    use glib::subclass::prelude::*;
    use glib::Sender;
    use gtk;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use inox_core::database::Thread;
    use once_cell::unsync::OnceCell;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct MessagesView {
        pub list_box: gtk::ListBox,
        pub rows: RefCell<Vec<MessageRowBase>>,
        // pub column_view: gtk::ColumnView,
        // pub model: gio::ListStore,
        // pub filter: gtk::TreeModelFilter,
        // idle_handle: RefCell<Option<glib::SourceId>>,
        // thread_list: RefCell<Option<Threads>>,

        // num_threads: u32,
        // num_threads_loaded: u32
        pub sender: OnceCell<Sender<Action>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MessagesView {
        const NAME: &'static str = "InoxMessagesView";
        type Type = super::MessagesView;
        type ParentType = gtk::Widget;

        fn new() -> Self {
            Self {
                list_box: gtk::ListBox::new(),
                rows: RefCell::new(vec![]),
                sender: OnceCell::new(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            // klass.set_layout_manager_type::<gtk::BinLayout>();
        }
    }

    impl ObjectImpl for MessagesView {
        fn constructed(&self, obj: &Self::Type) {
            self.list_box.set_parent(obj);
            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {
            self.list_box.unparent();

            let mut rows = self.rows.borrow_mut();
            for row in rows.iter() {
                row.unparent();
            }
        }
    }
    impl WidgetImpl for MessagesView {}
}

// Wrap imp::MessagesView into a usable gtk-rs object
glib::wrapper! {
    pub struct MessagesView(ObjectSubclass<imp::MessagesView>)
        @extends gtk::Widget;
}

// MessagesView implementation itself
impl MessagesView {
    pub fn new(thread: &notmuch::Thread, sender: Sender<Action>) -> Self {
        let view: Self = glib::Object::new(&[]).expect("Failed to create MessagesView");
        let imp = imp::MessagesView::from_instance(&view);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on MessagesView");
        view.set_vexpand(true);
        view.set_vexpand_set(true);
        view.load_messages(thread);
        view
    }

    pub fn load_messages(&self, thread: &notmuch::Thread) {
        let mut messages = thread.toplevel_messages();
        for message in messages {
            self.add_message(&message);
        }
    }

    pub fn add_message(&self, message: &notmuch::Message) {
        let imp = imp::MessagesView::from_instance(self);
        let message_row = MessageRow::new(message, imp.sender.get().unwrap().clone());
        imp.list_box.append(&message_row);
        imp.rows
            .borrow_mut()
            .push(message_row.upcast::<MessageRowBase>());
    }

    pub fn clear(&self) {
        // self.list_box.foreach(|child| {
        //     self.list_box.remove(&child);
        // });
    }
}
