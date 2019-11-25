
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use cursive::Cursive;
use cursive::views::{ViewRef, ListView};
use cursive::view::Identifiable;

use enamel_core::settings::Settings;
use enamel_core::database::Manager as DBManager;

use notmuch;

use crate::thread_list::ThreadListView;


pub struct EnamelApp {
    settings: Rc<Settings>,
    dbmanager: Rc<DBManager>
}

impl EnamelApp {

    pub fn new(siv: &mut Cursive, settings: Rc<Settings>) -> Self {

        let dbmanager = Rc::new(DBManager::new(&settings));

        // TODO: get this from config
        let initial_search = "tag:encrypted AND NOT tag:killed";
        let initial_command = format!("search {}", initial_search);


        // first build the UI, then init all the rest??
        siv.add_layer(ThreadListView::new(dbmanager.clone()).with_id("ThreadList"));



        let db = dbmanager.get(notmuch::DatabaseMode::ReadOnly).unwrap();
        let query = Arc::new(<notmuch::Database as notmuch::DatabaseExt>::create_query(db.clone(), &initial_search).unwrap());

        let mut thread_list: ViewRef<ThreadListView> = siv.find_id("ThreadList").unwrap();
        thread_list.set_query(query);






        Self{
            settings,
            dbmanager
        }
    }


}