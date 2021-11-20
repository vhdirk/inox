use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crate::settings::Settings;
use notmuch;

pub struct Manager {
    notmuch_db_path: PathBuf,
    database: RefCell<Option<notmuch::Database>>,
}

impl Manager {
    pub fn new(settings: &Rc<Settings>) -> Self {
        Manager {
            notmuch_db_path: PathBuf::from(settings.notmuch_config.database.path.clone()),
            database: RefCell::new(None),
        }
    }

    // get a database handle in the current mode
    pub fn get(
        &self,
        mode: notmuch::DatabaseMode,
    ) -> Result<notmuch::Database, notmuch::Error> {
        let current_db = self.database.borrow().clone();
        let open_new = match current_db {
            Some(_db) => false,
            None => true,
        };

        if open_new {
            // TODO: timeouts?
            let database = notmuch::Database::open(&self.notmuch_db_path, mode).unwrap();
            self.database.replace(Some(database.clone()));
        };

        Ok(self.database.borrow().clone().unwrap())
    }

}
