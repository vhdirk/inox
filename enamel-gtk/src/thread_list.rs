use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use gio;
use glib;
use glib::value::AnyValue;
use glib::prelude::*;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;
use relm;
use relm_attributes::widget;
use relm::ToGlib;

use notmuch;
use notmuch::DatabaseMode;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;
use inox_core::database::Thread;

use thread_list_cell_renderer::CellRendererThread;

const COLUMN_ID:u8 = 0;
const COLUMN_THREAD:u8 = 1;
const COLUMN_AUTHORS:u8 = 2;


fn append_text_column(tree: &gtk::TreeView, id: i32, title: &str) {
    let column = gtk::TreeViewColumn::new();
    let cell = CellRendererThread::new();

    column.pack_start(&cell, false);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "thread", id);
    column.set_title(&title);
    tree.append_column(&column);
}

pub fn gtk_idle_add<F: Fn() -> MSG + 'static, MSG: 'static>(stream: &::relm::EventStream<MSG>, constructor: F, single_shot:Option<bool>) -> glib::source::SourceId {
    let stream = stream.clone();
    gtk::idle_add(move || {
        let msg = constructor();
        stream.emit(msg);
        Continue(!single_shot.unwrap_or(false))
    })
}





#[derive(Msg, Debug)]
pub enum Msg {
    // outbound
    ThreadSelect(Option<String>),

    // inbound
    /// signals a request to update the event list. String is a notmuch query string
    Update(String),

    // private
    ItemSelect,
    AsyncFetch(AsyncFetchEvent)
}

#[derive(Debug)]
pub enum AsyncFetchEvent{
    Init,
    // NewItem,
    Complete,
    // Fail
}


pub struct ThreadList{
    model: ThreadListModel,
    scrolled_window: gtk::ScrolledWindow,
    tree_view: gtk::TreeView,
    tree_filter: gtk::TreeModelFilter,
    tree_model: gtk::ListStore

}

pub struct ThreadListModel {
    relm: ::relm::Relm<ThreadList>,
    settings: Rc<Settings>,
    dbmanager: Arc<DBManager>,

    idle_handle: Option<glib::SourceId>,
    thread_list: Option<notmuch::Threads>,

    num_threads: u32,
    num_threads_loaded: u32
}



fn create_liststore() -> gtk::ListStore{
    gtk::ListStore::new(&[String::static_type(), AnyValue::static_type()])
}

impl ThreadList{

    fn update(&mut self, qs: String){

        if self.model.idle_handle.is_some(){
            glib::source::source_remove(self.model.idle_handle.take().unwrap());
        }
        self.tree_model = create_liststore();
        self.tree_view.set_model(&self.tree_model);



        let mut dbman = self.model.dbmanager.clone();
        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();

        let query = db.create_query(&qs).unwrap();


        self.model.thread_list = Some(query.search_threads().unwrap());


        // let do_run = run.clone();
        gtk::idle_add(move || {
            debug!("thread count: {:?}", query.count_threads().unwrap());
            Continue(false)
        });


        gtk_idle_add(self.model.relm.stream(), || Msg::AsyncFetch(AsyncFetchEvent::Init), Some(true));

    }


    fn add_thread(&mut self, thread: Thread){

        let val = AnyValue::new(thread.clone()).to_value();

        let subject = &thread.subject();
        self.tree_model.insert_with_values(None,
            &[COLUMN_ID as u32,
              COLUMN_THREAD as u32
            ],
            &[&thread.id().to_value(),
              &val
            ]);
    }

    fn next_thread(&mut self){
        if self.model.thread_list.is_none(){

            return ();
        }

        match self.model.thread_list.as_mut().unwrap().next() {
            Some(mthread) => {
                gtk_idle_add(self.model.relm.stream(), || Msg::AsyncFetch(AsyncFetchEvent::Init), Some(true));
                self.add_thread(mthread.into());

            },
            None => {

            }
        }

    }


}


impl ::relm::Update for ThreadList {
    type Model = ThreadListModel;
    type ModelParam = (Rc<Settings>, Arc<DBManager>);
    type Msg = Msg;

    fn model(relm: &::relm::Relm<Self>, (settings, dbmanager): Self::ModelParam) -> Self::Model {
        ThreadListModel {
            relm: relm.clone(),
            settings,
            dbmanager,

            thread_list: None,
            idle_handle: None,
            num_threads: 0,
            num_threads_loaded: 0
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Update(ref qs) => self.update(qs.clone()),
            Msg::ItemSelect => {
                let selection = self.tree_view.get_selection();
                if let Some((list_model, iter)) = selection.get_selected() {
                    let thread_id: String = list_model.get_value(&iter, COLUMN_ID as i32)
                                                      .get::<String>()
                                                      .unwrap();

                    debug!("select thread: {:?}", thread_id);
                    self.model.relm.stream().clone().emit(Msg::ThreadSelect(Some(thread_id)));
                }
            },
            Msg::ThreadSelect(ref _thread_id) => (),
            Msg::AsyncFetch(AsyncFetchEvent::Init) => self.next_thread(),
            Msg::AsyncFetch(AsyncFetchEvent::Complete) => ()

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
        let tree_model = create_liststore();
        let tree_filter = gtk::TreeModelFilter::new(&tree_model, None);
        let tree_view = gtk::TreeView::new();


        tree_view.set_headers_visible(false);
        append_text_column(&tree_view, COLUMN_THREAD as i32, "Thread");

        scrolled_window.add(&tree_view);

        connect!(relm, tree_view, connect_cursor_changed(_), Msg::ItemSelect);

        ThreadList {
            model,
            scrolled_window,
            tree_view,
            tree_filter,
            tree_model
        }
    }
}
