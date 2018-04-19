use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use scoped_pool::Pool;

use gio;
use glib;
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

use thread_list_item::ThreadListItem;

fn append_text_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

pub fn gtk_idle_add<F: Fn() -> MSG + 'static, MSG: 'static>(stream: &::relm::EventStream<MSG>, constructor: F) -> glib::source::SourceId {
    let stream = stream.clone();
    gtk::idle_add(move || {
        let msg = constructor();
        stream.emit(msg);
        Continue(true)
    })
}



#[derive(Msg, Debug)]
pub enum Msg {
    // outbound
    ItemSelect,

    // inbound
    /// signals a request to update the event list. String is a notmuch query string
    Update(String),

    // private
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

    thread_list: Option<notmuch::Threads>,

    num_threads: u32,
    num_threads_loaded: u32
}



#[derive(Default, Debug)]
struct MailThread {
    pub id: String,
    pub subject: String,
    pub total_messages: i32,
    pub authors: Vec<String>,
    pub oldest_date: i64,
    pub newest_date: i64
}

#[derive(Debug)]
enum ChannelItem{
    Thread(MailThread),
    Count(u32),
}


fn add_thread(tree_model: gtk::ListStore, thread: MailThread){

    let subject = &thread.subject;
    let it = tree_model.append();
    tree_model.set_value(&it, 0, &thread.subject.to_value());

}

impl ThreadList{

    fn update(&mut self, qs: String){

        // if self.model.async_handle.is_some(){
        //     let async_handle = self.model.async_handle.take().unwrap();
        //     async_handle.run.store(false, Ordering::Relaxed);
        //     async_handle.join_handle.join().unwrap();
        //
        //     // TODO: how do we test if the idle handle is actually correct?
        //     glib::source::source_remove(async_handle.idle_handle);
        // }

        let mut dbman = self.model.dbmanager.clone();
        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();


        let query = db.create_query(&qs).unwrap();


        self.model.thread_list = Some(query.search_threads().unwrap());



        // let do_run = run.clone();
        gtk::idle_add(move || {
            debug!("thread count: {:?}", query.count_threads().unwrap());
            Continue(false)
        });


        let idle_handle = gtk_idle_add(self.model.relm.stream(), || Msg::AsyncFetch(AsyncFetchEvent::Init));

    }


    fn add_thread(&mut self, thread: notmuch::Thread){

        let subject = &thread.subject();
        let it = self.tree_model.append();
        self.tree_model.set_value(&it, 0, &thread.subject().to_value());

    }

    fn next_thread(&mut self){
        if self.model.thread_list.is_none(){
            return;
        }

        match self.model.thread_list.as_mut().unwrap().next() {
            Some(mthread) => {
                self.add_thread(mthread);
            },
            None => ()
        }

    }


    // fn async_fetch_stop(&mut self){
    //     if self.model.async_handle.is_some(){
    //         let async_handle = self.model.async_handle.as_mut().unwrap();
    //
    //         // TODO: how do we test if the idle handle is actually correct?
    //         glib::source::source_remove(glib::translate::FromGlib::from_glib(async_handle.idle_handle.to_glib().clone()));
    //     }
    // }

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
            num_threads: 0,
            num_threads_loaded: 0
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Update(ref qs) => self.update(qs.clone()),
            Msg::ItemSelect => (),
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

        let tree_model = gtk::ListStore::new(&[String::static_type()]);
        let tree_filter = gtk::TreeModelFilter::new(&tree_model, None);
        let tree_view = gtk::TreeView::new_with_model(&tree_model);


        tree_view.set_headers_visible(false);
        append_text_column(&tree_view, 0);

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
