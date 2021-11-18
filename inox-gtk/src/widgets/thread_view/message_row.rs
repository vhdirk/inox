use crate::app::Action;
use glib::Sender;
use inox_core::database::Thread;

use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, prelude::*, subclass::prelude::*};

use super::message_row_base::{MessageRowBase, MessageRowBaseImpl};

mod imp {

    use crate::app::Action;
    use glib::Sender;
    use glib::{self, prelude::*, subclass::prelude::*};
    use gtk::{self, prelude::*, subclass::prelude::*};
    use once_cell::unsync::OnceCell;

    #[derive(Debug)]
    pub struct MessageRow {
        pub sender: OnceCell<Sender<Action>>,
        pub message: Option<notmuch::Message>,
        pub spinner: gtk::Spinner,
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
                message: None,
                spinner: gtk::Spinner::new(),
                is_expanded: false,
            }
        }
    }

    impl ObjectImpl for MessageRow {

        fn constructed(&self, obj: &Self::Type) {
            // get_style_context().add_class(LOADING_CLASS);

            self.spinner.set_height_request(16);
            self.spinner.set_width_request(16);
            self.spinner.show();
            self.spinner.start();
            self.spinner.set_parent(obj);

            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {
            self.spinner.unparent();
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
            .set(sender)
            .expect("Failed to set sender on MessageRow");
        row.set_vexpand(true);
        row.set_vexpand_set(true);





        row
    }
}