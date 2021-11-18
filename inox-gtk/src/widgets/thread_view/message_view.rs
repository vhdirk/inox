use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use inox_core::database::Thread;

use notmuch;

use crate::app::Action;

mod imp {
    use crate::app::Action;
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::Sender;
    use gtk::{self, prelude::*, subclass::prelude::*, CompositeTemplate};
    use once_cell::unsync::OnceCell;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Inox/gtk/message_view.ui")]
    pub struct MessageView {
        #[template_child]
        pub actions: TemplateChild<gtk::Grid>,

        #[template_child]
        pub attachments_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub star_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub unstar_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub email_menubutton: TemplateChild<gtk::MenuButton>,

        #[template_child]
        pub sub_messages: TemplateChild<gtk::Grid>,

        pub sender: OnceCell<Sender<Action>>,
    }

    impl MessageView {}

    #[glib::object_subclass]
    impl ObjectSubclass for MessageView {
        const NAME: &'static str = "InoxMessageView";
        type Type = super::MessageView;
        type ParentType = gtk::Widget;

        fn new() -> Self {
            Self {
                actions: TemplateChild::default(),
                attachments_button: TemplateChild::default(),
                star_button: TemplateChild::default(),
                unstar_button: TemplateChild::default(),
                email_menubutton: TemplateChild::default(),
                sub_messages: TemplateChild::default(),
                sender: OnceCell::new(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MessageView {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {
            // self.list_box.unparent();
        }
    }
    impl WidgetImpl for MessageView {}
}

// Wrap imp::MessageView into a usable gtk-rs object
glib::wrapper! {
    pub struct MessageView(ObjectSubclass<imp::MessageView>)
        @extends gtk::Widget;
}

// MessageView implementation itself
impl MessageView {
    pub fn new(sender: Sender<Action>) -> Self {
        let thread_list: Self = glib::Object::new(&[]).expect("Failed to create MessageView");
        let imp = imp::MessageView::from_instance(&thread_list);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on MessageView");

        thread_list
    }
}
