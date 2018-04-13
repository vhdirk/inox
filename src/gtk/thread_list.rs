use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;
use relm;
use relm_attributes::widget;

use notmuch;
use notmuch::DatabaseMode;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use thread_list_item::ThreadListItem;

fn append_text_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}


#[derive(Msg)]
pub enum Msg {
    // outbound
    ItemSelect,

    // inbound
    Update(String),

    // private
    AddThreads
}

pub struct ThreadList {
    model: ThreadListModel,
    scrolled_window: gtk::ScrolledWindow,
    tree_view: gtk::TreeView,
    tree_filter: gtk::TreeModelFilter,
    tree_model: gtk::ListStore
}

pub struct ThreadListModel {
    relm: ::relm::Relm<ThreadList>,
    settings: Rc<Settings>,
    dbmanager: Rc<DBManager>,
}

impl ThreadList{

    fn update(&mut self, qs: String){
        self.tree_model.clear();

        let mut dbman = self.model.dbmanager.clone();
        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();

        let query = db.create_query(&qs).unwrap();

        let mut threads = Arc::new(RwLock::new(query.search_threads().unwrap()));

        let mut thread_list = threads.clone();

        let (tx, rx) = channel();

        thread::spawn(move || {

            let mut thrds = thread_list.write().unwrap();
            loop {
                match thrds.next() {
                    Some(thread) => {
                        // let thrd = Arc::new(RwLock::new(thread));
                        tx.send(thread);
                    },
                    None => { break }
                }
            }


            // let thread_list: Vec<notmuch::Thread> = threads.collect();
            // tx.send(thread_list);

        });


        // ::relm::interval(self.model.relm.stream(), 500, Msg::AddThreads);

        gtk::timeout_add(500, move || {
            let threa = rx.recv().unwrap();

            println!("{:?}", threa);

            Continue(false)
        });


        // loop {
        //     match threads.next() {
        //         Some(thread) => {
        //             self.add_thread(&thread);
        //         },
        //         None => { break }
        //     }
        // }
    }

    fn add_thread(self: &mut Self, thread: &notmuch::Thread){
        // debug!("thread {:?} {:?}", thread.subject(), thread.authors());
        //
        //
        let subject = &thread.subject();
        //let it = self.tree_model.append();
        //self.tree_model.set_value(&it, 0, &"".to_string().to_value());

    }
}


impl ::relm::Update for ThreadList {
    type Model = ThreadListModel;
    type ModelParam = (Rc<Settings>, Rc<DBManager>);
    type Msg = Msg;

    fn model(relm: &::relm::Relm<Self>, (settings, dbmanager): Self::ModelParam) -> Self::Model {
        ThreadListModel {
            relm: relm.clone(),
            settings,
            dbmanager
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Update(ref qs) => self.update(qs.clone()),
            Msg::ItemSelect => (),
            Msg::AddThreads => ()
        }
    }
}


impl ::relm::Widget for ThreadList {

    type Root = gtk::ScrolledWindow;

    fn root(&self) -> Self::Root {
        self.scrolled_window.clone()
    }

    fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self
    {
        let scrolled_window = gtk::ScrolledWindow::new(None, None);

        let tree_model = gtk::ListStore::new(&[String::static_type()]);
        let tree_filter = gtk::TreeModelFilter::new(&tree_model, None);
        let tree_view = gtk::TreeView::new_with_model(&tree_filter);
        // let tree_view = gtk::TreeView::new();

        tree_view.set_headers_visible(false);
        append_text_column(&tree_view, 0);

        scrolled_window.add(&tree_view);

        connect!(relm, tree_view, connect_cursor_changed(_), Msg::ItemSelect);

        ThreadList {
            model,
            scrolled_window,
            tree_view,
            tree_filter,
            tree_model,
        }
    }
}
