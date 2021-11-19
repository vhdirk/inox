use crate::app::Action;
use glib::Sender;
use inox_core::database::Thread;

use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};

use super::message_row_base::{MessageRowBase, MessageRowBaseImpl};
use super::message_view::MessageView;
mod imp {

    use crate::app::Action;
    use glib::Sender;
    use glib::{self, prelude::*, subclass::prelude::*};
    use gtk::{self, prelude::*, subclass::prelude::*};
    use once_cell::unsync::OnceCell;

    #[derive(Debug)]
    pub struct MessageRow {
        pub sender: OnceCell<Sender<Action>>,
        pub message: OnceCell<notmuch::Message>,
        pub is_expanded: bool,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MessageRow {
        const NAME: &'static str = "InoxMessageRow";
        type Type = super::MessageRow;
        type ParentType = super::MessageRowBase;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        fn new() -> Self {
            Self {
                sender: OnceCell::new(),
                message: OnceCell::new(),
                is_expanded: false,
            }
        }
    }

    impl ObjectImpl for MessageRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {
            // if Some(view) = .spinner.unparent();
        }
    }
    impl WidgetImpl for MessageRow {}
    impl ListBoxRowImpl for MessageRow {}
    impl super::MessageRowBaseImpl for MessageRow {}
}

glib::wrapper! {
    pub struct MessageRow(ObjectSubclass<imp::MessageRow>)
    @extends MessageRowBase, gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MessageRow {
    pub fn new(message: &notmuch::Message, sender: Sender<Action>) -> Self {
        let row: Self = glib::Object::new(&[]).expect("Failed to create MessageRow");
        let imp = imp::MessageRow::from_instance(&row);

        imp.sender
            .set(sender.clone())
            .expect("Failed to set sender on MessageRow");
        row.set_vexpand(true);
        row.set_vexpand_set(true);

        imp.message.set(message.clone())
            .expect("Failed to set message on MessageRow");

        let view = MessageView::new(message, sender);

        view.set_parent(&row);

        row
    }
}
