use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use notmuch;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use notmuch::DatabaseMode;

pub struct ThreadList {
    pub container: gtk::TreeView,

    model: gtk::ListStore,
    dbmanager: Rc<DBManager>
}

fn append_text_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}


impl ThreadList {
    pub fn new(dbmanager: Rc<DBManager>) -> Self {

        let model = gtk::ListStore::new(&[//i64::static_type(), //newest_date
                                          //i64::static_type(), //oldest_date
                                          String::static_type(), //thread_id
                                         // bool::static_type() //marked
                                          ]);
        // Gtk::TreeModelColumn<time_t> newest_date;
        // Gtk::TreeModelColumn<time_t> oldest_date;
        // Gtk::TreeModelColumn<Glib::ustring> thread_id;
        // Gtk::TreeModelColumn<Glib::RefPtr<NotmuchThread>> thread;
        // Gtk::TreeModelColumn<bool> marked;

        let container = gtk::TreeView::new_with_model(&model);
        container.set_headers_visible(false);
        append_text_column(&container, 0);

        
        ThreadList {
            container,
            model,
            dbmanager
        }
    }

    pub fn refresh(self: &mut Self)//, query: &notmuch::Query)
    {

        self.model.clear();

        let mut dbman = self.dbmanager.clone();

        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();
        let query = db.create_query(&"from:vhdirk@gmail.com".to_string()).unwrap();

        let mut threads = query.search_threads().unwrap();


        loop {
            match threads.next() {
                Some(thread) => {
                    self.add_thread(&thread);
                },
                None => { break }
            }
        }


    }

    fn add_thread(self: &mut Self, thread: &notmuch::Thread){

        debug!("thread {:?} {:?}", thread.subject(), thread.authors());

        let it = self.model.append();
        self.model.set_value(&it, 0, &thread.subject().to_value());

    }
}
