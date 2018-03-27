use std::rc::Rc;
use std::borrow::Borrow;
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

pub struct TagList {
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


impl TagList {
    pub fn new(dbmanager: Rc<DBManager>) -> Self {

        let model = gtk::ListStore::new(&[String::static_type()]);


        let container = gtk::TreeView::new_with_model(&model);
        container.set_headers_visible(false);
        append_text_column(&container, 0);


        let mut tl = TagList {
            container,
            model,
            dbmanager,
        };

        return tl;
    }

    pub fn refresh(self: &mut Self){

        self.model.clear();

        let mut dbman = self.dbmanager.clone();

        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();

        let mut tags = db.all_tags().unwrap();

        loop {
            match tags.next() {
                Some(tag) => {
                    debug!("tags {:?}", tag);
                    self.add_tag(&tag);
                },
                None => { break }
            }
        }


    }

    fn add_tag(self: &mut Self, tag: &String){


        let it = self.model.append();
        self.model.set_value(&it, 0, &tag.to_value());

    }

}
