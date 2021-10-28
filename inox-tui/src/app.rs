
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use cursive::Cursive;
use cursive::views::{ViewRef, ListView};
use cursive::view::Identifiable;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use notmuch;

use crate::thread_list::ThreadListView;


pub struct InoxApp {
    settings: Rc<Settings>,
    dbmanager: Rc<DBManager>
}

impl InoxApp {

    pub fn new(siv: &mut Cursive, settings: Rc<Settings>) -> Self {

        let dbmanager = Rc::new(DBManager::new(&settings));

        // TODO: get this from config
        let initial_search = "tag:inbox AND NOT tag:killed";
        let initial_command = format!("search {}", initial_search);


        // first build the UI, then init all the rest??
        siv.add_layer(ThreadListView::new(dbmanager.clone()).with_id("ThreadList"));



        let db = dbmanager.get(notmuch::DatabaseMode::ReadOnly).unwrap();
        let query = Arc::new(notmuch::Query::create(db.clone(), &initial_search).unwrap());

        let mut thread_list: ViewRef<ThreadListView> = siv.find_id("ThreadList").unwrap();
        thread_list.set_query(query);






        Self{
            settings,
            dbmanager
        }
    }


}