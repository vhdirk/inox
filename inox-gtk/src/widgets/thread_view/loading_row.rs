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

    #[derive(Debug, Default)]
    pub struct LoadingRow {
        pub sender: OnceCell<Sender<Action>>,
        pub is_expanded: bool,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LoadingRow {
        const NAME: &'static str = "InoxLoadingRow";
        type Type = super::LoadingRow;
        type ParentType = super::MessageRowBase;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }
    }

    impl ObjectImpl for LoadingRow {

        fn constructed(&self, obj: &Self::Type) {
            // get_style_context().add_class(LOADING_CLASS);

            let spinner = gtk::Spinner::new();
            spinner.set_height_request(16);
            spinner.set_width_request(16);
            spinner.show();
            spinner.start();
            spinner.set_parent(obj);

            self.parent_constructed(obj);
        }

    }
    impl WidgetImpl for LoadingRow {}
    impl ListBoxRowImpl for LoadingRow {}
    impl super::MessageRowBaseImpl for LoadingRow {}
}

glib::wrapper! {
    pub struct LoadingRow(ObjectSubclass<imp::LoadingRow>)
    @extends MessageRowBase, gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl LoadingRow {
    pub fn new(sender: Sender<Action>) -> Self {
        let row: Self = glib::Object::new(&[]).expect("Failed to create LoadingRow");
        let imp = imp::LoadingRow::from_instance(&row);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on LoadingRow");
        row.set_vexpand(true);
        row.set_vexpand_set(true);

        row
    }
}