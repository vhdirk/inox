use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

use notmuch;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use tag_list::{TagList, Msg as TagListMsg};
use thread_list::{ThreadList, Msg as ThreadListMsg};
use thread_view::ThreadView;




// impl MainContent {
//     pub fn new(dbmanager: Rc<DBManager>) -> Self {
//         // Create the Paned container for the main content
//         let container = gtk::Paned::new(gtk::Orientation::Horizontal);
//         let mut tag_list = TagList::new(dbmanager.clone());
//
//         // TODO: make thread splitter orientation configurable
//         let thread_container = gtk::Paned::new(gtk::Orientation::Horizontal);
//
//
//         // TODO: refresh tag list only when we think it might be needed.
//         tag_list.refresh();
//
//         let mut thread_list = ThreadList::new(dbmanager.clone());
//         let mut thread_view = ThreadView::new();
//
//         thread_list.refresh();
//
//         thread_container.pack1(&thread_list.container, true, true);
//         thread_container.pack2(&thread_view.container, true, false);
//
//         container.pack1(&tag_list.container, true, true);
//         container.pack2(&thread_container, true, false);
//
//         thread_container.set_size_request(100, -1);
//         tag_list.container.set_size_request(100, -1);
//         thread_list.container.set_size_request(100, -1);
//         thread_view.container.set_size_request(100, -1);
//
//         MainContent {
//             container,
//             tag_list,
//             thread_list,
//             thread_view
//         }
//     }
// }

#[derive(Msg)]
pub enum MainContentMsg {
    TagSelect(String)
}


pub struct MainContentModel {
    relm: ::relm::Relm<MainContent>,
    ui_orientation: gtk::Orientation,
    settings: Rc<Settings>,
    dbmanager: Rc<DBManager>
}

#[widget]
impl ::relm::Widget for MainContent {
    type Model = MainContentModel;
    type ModelParam = (Rc<Settings>, Rc<DBManager>);
    type Msg = MainContentMsg;

    fn init_view(&mut self) {
        self.thread_container.set_size_request(100, -1);
        self.tag_list.widget().set_size_request(100, -1);
        self.thread_list.widget().set_size_request(100, -1);
        self.thread_view.widget().set_size_request(100, -1);

        self.tag_list.emit(TagListMsg::Refresh);
        self.thread_list.emit(ThreadListMsg::Update);
    }

    fn model(relm: &::relm::Relm<Self>, (settings, dbmanager): (Rc<Settings>, Rc<DBManager>)) -> MainContentModel {
        MainContentModel {
            relm: relm.clone(),
            ui_orientation: gtk::Orientation::Horizontal,
            settings,
            dbmanager
        }
    }


    fn update(&mut self, _event: MainContentMsg) {
        // self.label.set_text("");
    }

    view! {
        #[name="container"]
        gtk::Paned(gtk::Orientation::Horizontal) {
            #[name="tag_list"]
            TagList(self.model.settings.clone(), self.model.dbmanager.clone()),

            #[name="thread_container"]
            gtk::Paned(self.model.ui_orientation){
                #[name="thread_list"]
                ThreadList(self.model.settings.clone(), self.model.dbmanager.clone()),
                #[name="thread_view"]
                ThreadView
            }
        }
    }
}
