use gio::prelude::*;
use gtk::prelude::*;
use glib::subclass::prelude::*;
use glib::Sender;

use notmuch;

use crate::core::Action;


use super::messages_view_imp as imp;

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
        imp.thread
            .set(thread.clone())
            .expect("Failed to set thread on MessagesView");

        view.set_vexpand(true);
        view.set_hexpand(true);

        imp.init();

        view
    }
}
