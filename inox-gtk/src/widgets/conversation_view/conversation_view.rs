use inox_core::models::Conversation;
use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;

use notmuch;

use crate::widgets::MessageList;
use crate::core::Action;
use crate::core::ConversationObject;

use super::conversation_view_imp as imp;


// Wrap imp::ConversationView into a usable gtk-rs object
glib::wrapper! {
    pub struct ConversationView(ObjectSubclass<imp::ConversationView>)
        @extends gtk::Widget;
}

// ConversationView implementation itself
impl ConversationView {
    pub fn new(sender: Sender<Action>) -> Self {
        let view: Self = glib::Object::new(&[]).expect("Failed to create ConversationView");
        let imp = imp::ConversationView::from_instance(&view);

        imp.sender
            .set(sender)
            .expect("Failed to set sender on ConversationView");
        view.set_vexpand(true);
        view.set_vexpand_set(true);

        // view.setup_callbacks();

        view
    }

    /**
     * Shows the loading UI.
     */
    fn show_loading(&self) {
        let imp = imp::ConversationView::from_instance(self);

        imp.loading_page.get().start();
        imp.set_visible_child(&imp.loading_page.get());
    }

    fn setup_columns(&self) {
        let imp = imp::ConversationView::from_instance(self);
    }

    pub fn load_conversation(&self, conversation: &Conversation) {
        let imp = imp::ConversationView::from_instance(self);
        // self.show_loading();

        // let message_list = MessageList::new(thread, imp.sender.get().unwrap().clone());

        // // insert the new view
        // imp.set_message_list(&message_list);

        // imp.set_visible_child(&imp.thread_page.get());

        // let model = imp::create_liststore();
        // let selection_model = SingleSelection::new(Some(&model));

        // for thread in threads {
        //     model.append(&Thread::new(thread));
        // }

        // imp.column_view.set_model(Some(&selection_model));
    }
}
