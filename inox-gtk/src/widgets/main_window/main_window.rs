use inox_core::models::Message;
use inox_core::models::Conversation;
use std::cell::RefCell;
use std::sync::Arc;

use adw;
use adw::prelude::*;
use futures::future::FutureExt;
use gio::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::traits::{GtkWindowExt, WidgetExt};
use gtk::{prelude::*, CompositeTemplate};
use log::*;

use crate::core::Action;
use crate::application::InoxApplication;

// use crate::headerbar::HeaderBar;
use crate::core::ConversationObject;

use crate::widgets::conversation_view::ConversationView;
use crate::widgets::conversation_list::ConversationList;
use super::main_window_imp as imp;

// Wrap imp::MainWindow into a usable gtk-rs object
glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

// MainWindow implementation itself
impl MainWindow {
    pub fn new(sender: Sender<Action>, app: InoxApplication) -> Self {
        // Create new GObject and downcast it into MainWindow
        let window: Self = glib::Object::new(&[("application", &app)])
            .expect("Failed to create ApplicationWindow");

        let imp = imp::MainWindow::from_instance(&window);
        imp.sender
            .set(sender.clone())
            .expect("Failed to set sender on MessageRow");

        app.add_window(&window.clone());
        imp.init();
        // // window.setup_signals();
        // // window.setup_gactions(sender);
        window
    }


    // fn setup_signals(&self) {
    //     let imp = imp::MainWindow::from_instance(self);

    //     // get_widget!(imp.window_builder, gtk::Paned, main_header);
    //     // get_widget!(imp.window_builder, gtk::Paned, main_paned);

    //     let _width_bind = imp
    //         .main_paned
    //         .get()
    //         .bind_property("position", &imp.main_header.get(), "position")
    //         .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
    //         .transform_to(move |_binding, value| {
    //             let _offset = 0; //TODO: this offset was trial and error.
    //                              // we should calculate it somehow.
    //             Some((value.get::<i32>().unwrap_or(0)).to_value())
    //         })
    //         .build();

    //     // // window gets closed
    //     // self.connect_delete_event(move |window, _| {
    //     //     debug!("Saving window geometry.");
    //     //     // let _width = window.size().0;
    //     //     // let _height = window.size().1;

    //     //     // settings_manager::set_integer(Key::WindowWidth, width);
    //     //     // settings_manager::set_integer(Key::WindowHeight, height);
    //     //     gtk::Inhibit(false)
    //     // });
    // }

    // // fn setup_gactions(&self, _sender: Sender<Action>) {
    // //     // We need to upcast from MainWindow to gtk::ApplicationWindow, because MainWindow
    // //     // currently doesn't implement GLib.ActionMap, since it's not supported in gtk-rs for subclassing (13-01-2020)
    // //     let window = self.clone().upcast::<gtk::ApplicationWindow>();
    // //     let _app = window.application().unwrap();
    // // }

    pub fn set_conversations(&self, conversations: &Vec<Conversation>) {
        let imp = imp::MainWindow::from_instance(self);
        imp.conversation_list.get().unwrap().set_conversations(conversations);
    }

    pub fn open_conversation(&self, conversation: Option<Conversation>, messages: Vec<Message>) {
        let imp = imp::MainWindow::from_instance(self);

        match conversation {
            Some(conversation) => {
                // self.update_titlebar(Some(&conversation.subject()));

                let conversation_view = imp.conversation_view.get().unwrap();
                conversation_view.load_messages(&conversation, messages);
            }
            None => {
                // self.update_titlebar(None);
            }
        }
    }

    // pub fn update_titlebar(&self, title: Option<&str>) {
    //     let imp = imp::MainWindow::from_instance(self);
    //     get_widget!(imp.window_builder, gtk::HeaderBar, conversation_header);
    //     conversation_header.set_subtitle(title);
    // }
}

// #[derive(Msg)]
// pub enum Msg {
//     TagSelect(Option<String>),
//     ThreadSelect(Thread),
//     Change,
//     Quit,
// }

// #[derive(Clone)]
// pub struct Model {
//     relm: Relm<MainWindow>,
//     app: Rc<InoxApp>
// }

// #[derive(Clone)]
// struct Widgets {
//     headerbar: Component<HeaderBar>,
//     taglist: Component<TagList>,
//     Conversationlist: Component<ConversationList>,
//     Conversationview: Component<ConversationView>
// }

// // TODO: Factor out the hamburger menu
// // TODO: Make a proper state machine for the headerbar states
// pub struct MainWindow {
//     model: Model,
//     container: gtk::ApplicationWindow,
//     widgets: Widgets
// }

// impl MainWindow {

//     fn on_tag_changed(self: &mut Self, tag: Option<String>){

//         // TODO: build a new query and refresh the thread list.
//         let dbman = self.model.app.dbmanager.clone();
//         let db = dbman.get(DatabaseMode::ReadOnly).unwrap();

//         let qs = match tag{
//             Some(tag) => format!("tag:{}", tag).to_string(),
//             None => "".to_string()
//         };
//         debug!("qs: {:?}", qs);

//         let query = <notmuch::Database as notmuch::DatabaseExt>::create_query(db, &qs).unwrap();
//         let threads = <notmuch::Query<'_> as notmuch::QueryExt>::search_threads(query).unwrap();

//         self.widgets.Conversationlist.emit(ConversationListMsg::Update(Some(threads)));
//     }

//     fn on_thread_selected(self: &mut Self, thread: Thread){
//         self.widgets.Conversationview.emit(ConversationViewMsg::ShowThread(thread))
//     }

// }

// impl Update for MainWindow{
//     type Model = Model;
//     type ModelParam = Rc<InoxApp>;
//     type Msg = Msg;

//     fn model(relm: &Relm<Self>, app: Self::ModelParam) -> Model {
//         Self::Model {
//             relm: relm.clone(),
//             app
//         }
//     }

//     fn update(&mut self, event: Msg) {
//         match event {
//             Msg::TagSelect(tag) => self.on_tag_changed(tag),
//             Msg::ThreadSelect(thread) => self.on_thread_selected(thread),
//             Msg::Change => {
//                 // self.model.content = self.widgets.input.get_text()
//                 //                                        .expect("get_text failed")
//                 //                                        .chars()
//                 //                                        .rev()
//                 //                                        .collect();
//                 // self.widgets.label.set_text(&self.model.content);
//             },
//             Msg::Quit => gtk::main_quit(),
//         }
//     }
// }

// impl Widget for MainWindow {
//     type Root = gtk::ApplicationWindow;

//     fn root(&self) -> Self::Root {
//         self.container.clone()
//     }

//     fn view(relm: &Relm<Self>, model: Self::Model) -> Self {

//         let window = model.app.builder.get_object::<gtk::ApplicationWindow>("main_window")
//                                   .expect("Couldn't find main_window in ui file.");
//         window.set_application(Some(&model.app.instance));

//         let headerbar = relm_init::<HeaderBar>(model.app.clone()).unwrap();
//         let taglist = relm_init::<TagList>(model.app.clone()).unwrap();
//         let Conversationlist = relm_init::<ConversationList>(model.app.clone()).unwrap();
//         let Conversationview = relm_init::<ConversationView>(model.app.clone()).unwrap();

//         // TODO: what would be the best place to connect all UI signals?
//         use self::TagListMsg::ItemSelect as TagList_ItemSelect;
//         connect!(taglist@TagList_ItemSelect(ref tag), relm, Msg::TagSelect(tag.clone()));

//         use self::ConversationListMsg::ThreadSelect as ConversationList_ThreadSelect;
//         connect!(Conversationlist@ConversationList_ThreadSelect(ref thread), relm, Msg::ThreadSelect(thread.as_ref().unwrap().clone()));

//         MainWindow {
//             model,
//             container: window,
//             widgets: Widgets{
//                 headerbar,
//                 taglist,
//                 Conversationlist,
//                 Conversationview
//             }
//         }

//     }

//     fn init_view(&mut self) {

//         let main_paned = self.model.app.builder.get_object::<gtk::Paned>("main_paned")
//                                    .expect("Couldn't find main_paned in ui file.");

//         let Conversationlist_header = self.model.app.builder.get_object::<gtk::HeaderBar>("Conversationlist_header")
//                                  .expect("Couldn't find Conversationlist_header in ui file.");

//         // // TODO: do I need to unbind this at some point?
//         // let _width_bind = main_paned.bind_property("position", &Conversationlist_header, "width-request")
//         //                             .flags(glib::BindingFlags::SYNC_CREATE)
//         //                             .transform_to(move |_binding, value| {
//         //                                 let offset = 6; //TODO: this offset was trial and error.
//         //                                                 // we should calculate it somehow.
//         //                                 Some((value.get::<i32>().unwrap_or(Some(0)) + offset).to_value())
//         //                             })
//         //                             .build();

//         self.container.show_all();

//         self.widgets.taglist.emit(TagListMsg::Refresh);
//     }

// }
