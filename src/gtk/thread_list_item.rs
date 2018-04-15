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

use notmuch::DatabaseMode;


pub struct ThreadListItem{

}

// impl gtk::CellRendererExt for ThreadListItem{
//
// }

//
// fn append_text_column(tree: &gtk::TreeView, id: i32) {
//     let column = gtk::TreeViewColumn::new();
//     let cell = gtk::CellRendererText::new();
//
//     column.pack_start(&cell, true);
//     // Association of the view's column with the model's `id` column.
//     column.add_attribute(&cell, "text", id);
//     tree.append_column(&column);
// }
//
//
// #[derive(Msg)]
// pub enum Msg {
//     Update,
//     ItemSelect,
//     Refresh(String)
// }
//
// pub struct ThreadListItem {
//     model: ThreadListModel,
//     scrolled_window: gtk::ScrolledWindow,
//     tree_view: gtk::TreeView,
//     tree_model: gtk::ListStore
// }
//
// pub struct ThreadListItemModel {
//     relm: ::relm::Relm<ThreadList>,
//     settings: Rc<Settings>,
//     dbmanager: Arc<DBManager>,
// }
//
// impl ThreadListItem{
//     fn update(&mut self){
//         // self.tree_model.clear();
//         //
//         // let mut dbman = self.model.dbmanager.clone();
//         //
//         // let db = dbman.get(DatabaseMode::ReadOnly).unwrap();
//         // let query = db.create_query(&"from:vhdirk@gmail.com".to_string()).unwrap();
//         //
//         // let mut threads = query.search_threads().unwrap();
//         //
//         // loop {
//         //     match threads.next() {
//         //         Some(thread) => {
//         //             self.add_thread(&thread);
//         //         },
//         //         None => { break }
//         //     }
//         // }
//     }
//
//     fn refresh(&mut self, qs: String){
//         self.tree_model.clear();
//
//         let mut dbman = self.model.dbmanager.clone();
//
//         let db = dbman.get(DatabaseMode::ReadOnly).unwrap();
//         let query = db.create_query(&qs).unwrap();
//
//         let mut threads = query.search_threads().unwrap();
//
//         loop {
//             match threads.next() {
//                 Some(thread) => {
//                     self.add_thread(&thread);
//                 },
//                 None => { break }
//             }
//         }
//     }
//
//     fn add_thread(self: &mut Self, thread: &notmuch::Thread){
//         // debug!("thread {:?} {:?}", thread.subject(), thread.authors());
//         let it = self.tree_model.append();
//         self.tree_model.set_value(&it, 0, &thread.subject().to_value());
//     }
// }
//
//
// impl ::relm::Update for ThreadListItem {
//     type Model = ThreadListItemModel;
//     type ModelParam = (Rc<Settings>, Arc<DBManager>);
//     type Msg = Msg;
//
//     fn model(relm: &::relm::Relm<Self>, (settings, dbmanager): Self::ModelParam) -> Self::Model {
//         ThreadListModel {
//             relm: relm.clone(),
//             settings,
//             dbmanager
//         }
//     }
//
//     fn update(&mut self, event: Self::Msg) {
//         match event {
//             Msg::Update => self.update(),
//             Msg::ItemSelect => (),
//             Msg::Refresh(ref qs) => self.refresh(qs.clone())
//         }
//     }
// }
//
//
// impl ::relm::Widget for ThreadListItem {
//
//     type Root = gtk::ScrolledWindow;
//
//     fn root(&self) -> Self::Root {
//         self.scrolled_window.clone()
//     }
//
//     fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self
//     {
//         let scrolled_window = gtk::ScrolledWindow::new(None, None);
//
//         let tree_model = gtk::ListStore::new(&[String::static_type()]);
//         let tree_view = gtk::TreeView::new_with_model(&tree_model);
//         tree_view.set_headers_visible(false);
//         append_text_column(&tree_view, 0);
//
//         scrolled_window.add(&tree_view);
//
//         connect!(relm, tree_view, connect_cursor_changed(_), Msg::ItemSelect);
//
//         ThreadList {
//             model,
//             scrolled_window,
//             tree_view,
//             tree_model,
//         }
//     }
// }
