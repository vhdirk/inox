use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::path::{Path, PathBuf};

use std::collections::BTreeMap;
use toml;
use serde;

use notmuch;
use settings::Settings;

pub struct Manager{
    notmuch_db_path: PathBuf
}



impl Manager {

    pub fn new(settings: &Rc<Settings>) -> Self {


        Manager{
            notmuch_db_path: PathBuf::from(settings.notmuch_config.database.path.clone())
        }
    }

    // get a database handle in the current mode
    pub fn get(&self, mode: notmuch::DatabaseMode) -> Result<notmuch::Database, notmuch::Error>{

        // TODO: timeouts?

        notmuch::Database::open(&self.notmuch_db_path, mode)

    }


}
