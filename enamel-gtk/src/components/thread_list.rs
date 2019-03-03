use std::rc::Rc;
use std::sync::{Arc};

use log::*;
use glib;
use glib::prelude::*;
use gtk;
use gtk::prelude::*;
use glib::object::Object;
use glib::subclass::types::ObjectSubclass;
use relm::{Relm, EventStream, Widget, Update};
use relm_state::connect;
use relm_derive::Msg;

use notmuch;

use enamel_core::database::Thread;
use crate::app::EnamelApp;

type Threads = notmuch::Threads<'static, 'static>;

use crate::widgets::thread_list_cell_renderer::CellRendererThread;

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

pub fn gtk_idle_add<F: Fn() -> MSG + 'static, MSG: 'static>(stream: &EventStream<MSG>, constructor: F, single_shot:Option<bool>) -> glib::source::SourceId {
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
    ThreadSelect(Option<Thread>),

    // inbound
    /// signals a request to update the event list. String is a notmuch query string
    Update(Option<Threads>),

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
    relm: Relm<ThreadList>,
    app: Rc<EnamelApp>,

    idle_handle: Option<glib::SourceId>,
    thread_list: Option<Arc<Threads>>,

    num_threads: u32,
    num_threads_loaded: u32
}



fn create_liststore() -> gtk::ListStore{
    gtk::ListStore::new(&[String::static_type(), Thread::static_type()])
}

impl ThreadList{

    fn update(&mut self, threads: Option<Threads>){

        if self.model.idle_handle.is_some(){
            glib::source::source_remove(self.model.idle_handle.take().unwrap());
        }
        self.tree_model = create_liststore();
        self.tree_view.set_model(&self.tree_model);

        self.model.thread_list = threads.map(Arc::new);


        // // let do_run = run.clone();
        // gtk::idle_add(move || {
        //     debug!("thread count: {:?}", query.count_threads().unwrap());
        //     Continue(false)
        // });


        gtk_idle_add(self.model.relm.stream(), || Msg::AsyncFetch(AsyncFetchEvent::Init), Some(true));

    }


    fn add_thread(&mut self, thread: Thread){

        let thread_id = thread.id().clone();

        self.tree_model.insert_with_values(None,
            &[COLUMN_ID as u32,
              COLUMN_THREAD as u32
            ],
            &[&thread_id.to_value(),
              &thread
            ]);
    }

    fn next_thread(&mut self){
        if self.model.thread_list.is_none(){
            return ();
        }

        if let Some(thread) = Arc::get_mut(self.model.thread_list.as_mut().unwrap()).unwrap().next(){
            gtk_idle_add(self.model.relm.stream(), || Msg::AsyncFetch(AsyncFetchEvent::Init), Some(true));
            self.add_thread(Thread::new(thread));
        }

    }


}


impl Update for ThreadList {
    type Model = ThreadListModel;
    type ModelParam = Rc<EnamelApp>;
    type Msg = Msg;

    fn model(relm: &Relm<Self>, app: Self::ModelParam) -> Self::Model {
        ThreadListModel {
            relm: relm.clone(),
            app,

            thread_list: None,
            idle_handle: None,
            num_threads: 0,
            num_threads_loaded: 0
        }
    }

    fn update(&mut self, msg: Self::Msg) {
        match msg {
            Msg::Update(threads) => self.update(threads),
            Msg::ItemSelect => {
                let selection = self.tree_view.get_selection();
                if let Some((list_model, iter)) = selection.get_selected() {
                    let lval = list_model.get_value(&iter, COLUMN_THREAD as i32);
                    let thread = lval.get::<&Thread>().unwrap();

                    debug!("select thread: {:?}", thread);
                    self.model.relm.stream().clone().emit(Msg::ThreadSelect(Some(thread.clone())));
                }
            },
            Msg::ThreadSelect(ref _thread_id) => (),
            Msg::AsyncFetch(AsyncFetchEvent::Init) => self.next_thread(),
            Msg::AsyncFetch(AsyncFetchEvent::Complete) => ()

        }
    }
}


impl Widget for ThreadList {

    type Root = gtk::ScrolledWindow;

    fn root(&self) -> Self::Root {
        self.scrolled_window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self
    {
        let scrolled_window = model.app.builder.get_object::<gtk::ScrolledWindow>("thread_list_scrolled")
                                               .expect("Couldn't find thread_list_scrolled in ui file.");

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
