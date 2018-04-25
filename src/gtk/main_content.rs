use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;

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
//     pub fn new(dbmanager: Arc<DBManager>) -> Self {
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
pub enum Msg {
    TagSelect(Option<String>),
    ThreadSelect(Option<String>),
}


pub struct MainContentModel {
    relm: ::relm::Relm<MainContent>,
    ui_orientation: gtk::Orientation,
    settings: Rc<Settings>,
    dbmanager: Arc<DBManager>
}

impl MainContent {

    fn on_tag_changed(self: &mut Self, tag:Option<String>){


        // TODO: build a new query and refresh the thread list.

        let qs = match tag{
            Some(tag) => format!("tag:{}", tag).to_string(),
            None => "".to_string()
        };
        debug!("qs: {:?}", qs);


        self.thread_list.emit(ThreadListMsg::Update(qs));
    }

}

use self::Msg::TagSelect;
use self::Msg::ThreadSelect;
use self::ThreadListMsg::ThreadSelect as ThreadList_ThreadSelect;

use self::TagListMsg::ItemSelect;

#[widget]
impl ::relm::Widget for MainContent {
    type Model = MainContentModel;
    type ModelParam = (Rc<Settings>, Arc<DBManager>);
    type Msg = Msg;


    fn init_view(&mut self) {
        self.thread_container.set_size_request(100, -1);
        self.tag_list.widget().set_size_request(100, -1);
        self.thread_list.widget().set_size_request(100, -1);
        self.thread_view.widget().set_size_request(100, -1);

        self.tag_list.emit(TagListMsg::Refresh);

        self.on_tag_changed(None);
    }

    fn model(relm: &::relm::Relm<Self>, (settings, dbmanager): (Rc<Settings>, Arc<DBManager>)) -> MainContentModel {
        MainContentModel {
            relm: relm.clone(),
            ui_orientation: gtk::Orientation::Horizontal,
            settings,
            dbmanager
        }
    }


    fn update(&mut self, event: Msg) {
        match event {
            Msg::TagSelect(tag) => self.on_tag_changed(tag),
            Msg::ThreadSelect(ref thread_id) => {
                debug!("select thread: {:?}", thread_id);

            },
        }
    }

    view! {
        #[name="container"]
        gtk::Paned(gtk::Orientation::Horizontal) {
            #[name="tag_list"]
            TagList((self.model.settings.clone(), self.model.dbmanager.clone())){
                ItemSelect(ref tag) => TagSelect(tag.clone())
            },

            #[name="thread_container"]
            gtk::Paned(self.model.ui_orientation){
                #[name="thread_list"]
                ThreadList(self.model.settings.clone(), self.model.dbmanager.clone()) {
                    ThreadList_ThreadSelect(ref thread_id) => ThreadSelect(thread_id.clone()),
                },
                #[name="thread_view"]
                ThreadView
            }
        }
    }
}
