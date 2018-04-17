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


struct AsyncThreadHandle {
    pub join_handle: thread::JoinHandle<()>,
    pub idle_handle: glib::source::SourceId,
    pub run: Arc<AtomicBool>,
    pub rx: Receiver<ChannelItem>
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

    async_handle: Option<AsyncThreadHandle>,

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

        if self.model.async_handle.is_some(){
            let async_handle = self.model.async_handle.take().unwrap();
            async_handle.run.store(false, Ordering::Relaxed);
            async_handle.join_handle.join().unwrap();

            // TODO: how do we test if the idle handle is actually correct?
            glib::source::source_remove(async_handle.idle_handle);
        }

        let mut dbman = self.model.dbmanager.clone();
        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();


        let (tx, rx): (Sender<ChannelItem>, Receiver<ChannelItem>)  = channel();

        let run = Arc::new(AtomicBool::new(true));

        let do_run = run.clone();

        let thread_handle = thread::spawn(move || {

            let query = db.create_query(&qs).unwrap();

            tx.send(ChannelItem::Count(query.count_threads().unwrap())).unwrap();

            let mut threads = query.search_threads().unwrap();

            while do_run.load(Ordering::Relaxed) {
                match threads.next() {
                    Some(mthread) => {
                        tx.send(ChannelItem::Thread(MailThread{
                            id: mthread.id(),
                            subject: mthread.subject(),
                            total_messages: mthread.total_messages(),
                            authors: mthread.authors(),
                            oldest_date: mthread.oldest_date(),
                            newest_date: mthread.newest_date()

                        })).unwrap();
                    },
                    None => { break }
                }
            }

        });


        let tree_model = gtk::ListStore::new(&[String::static_type()]);

        self.tree_view.set_model(&tree_model);
        self.tree_model = tree_model;

        // gtk::idle_add(move ||{
        //     tree_model.clear();
        //     Continue(false)
        // });

        let idle_handle = gtk_idle_add(self.model.relm.stream(), || Msg::AsyncFetch(AsyncFetchEvent::Init));

        self.model.async_handle = Some(AsyncThreadHandle{
            join_handle: thread_handle,
            idle_handle: idle_handle,
            run: run,
            rx: rx
        });

    }


    fn add_thread(&mut self, thread: MailThread){

        let subject = &thread.subject;
        let it = self.tree_model.append();
        self.tree_model.set_value(&it, 0, &thread.subject.to_value());

    }

    fn async_fetch_thread(&mut self){
        match self.model.async_handle.as_mut().unwrap().rx.try_recv(){
         Ok(ChannelItem::Thread(thread)) => {
             self.add_thread(thread);
             // Continue(true)
         },
         Ok(ChannelItem::Count(num)) => {
            println!("{:?} threads", num);
             // Continue(true)
         },
         Err(err) if err == TryRecvError::Empty => {

             // Continue(true)
         },
         Err(err) => {
            self.model.relm.stream().emit(Msg::AsyncFetch(AsyncFetchEvent::Complete));

             // Continue(false)
         }
        }
    }

    fn async_fetch_stop(&mut self){
        if self.model.async_handle.is_some(){
            let async_handle = self.model.async_handle.as_mut().unwrap();

            // TODO: how do we test if the idle handle is actually correct?
            glib::source::source_remove(glib::translate::FromGlib::from_glib(async_handle.idle_handle.to_glib().clone()));
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

            async_handle: None,

            num_threads: 0,
            num_threads_loaded: 0
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Update(ref qs) => self.update(qs.clone()),
            Msg::ItemSelect => (),
            Msg::AsyncFetch(AsyncFetchEvent::Init) => self.async_fetch_thread(),
            Msg::AsyncFetch(AsyncFetchEvent::Complete) => self.async_fetch_stop()

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
