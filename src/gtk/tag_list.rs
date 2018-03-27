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
    pub container: gtk::ListBox,

    dbmanager: Rc<DBManager>
}



impl TagList {
    pub fn new(dbmanager: Rc<DBManager>) -> Self {

        let container = gtk::ListBox::new();

        TagList {
            container,
            dbmanager
         }
    }

    pub fn refresh(self: &mut Self){

        let dbman:&DBManager = self.dbmanager.borrow();
        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();

        let mut tags = db.all_tags().unwrap();
        debug!("tags {:?}", &tags.next());

    }

}
