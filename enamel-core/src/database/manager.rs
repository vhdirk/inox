use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use std::collections::BTreeMap;
use toml;
use serde;

use notmuch;
use crate::settings::Settings;

pub struct Manager{
    notmuch_db_path: PathBuf,
    database: RefCell<Option<Arc<notmuch::Database>>>
}



impl Manager {

    pub fn new(settings: &Rc<Settings>) -> Self {

        Manager{
            notmuch_db_path: PathBuf::from(settings.notmuch_config.database.path.clone()),
            database: RefCell::new(None)
        }
    }

    // get a database handle in the current mode
    pub fn get(&self, mode: notmuch::DatabaseMode) -> Result<Arc<notmuch::Database>, notmuch::Error>{

        // TODO: timeouts?
        let database = Arc::new(notmuch::Database::open(&self.notmuch_db_path, mode).unwrap());
        self.database.replace(Some(database.clone()));
        Ok(database)
    }
}
