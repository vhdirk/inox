use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};

use std::collections::BTreeMap;
use toml;
use serde;

mod some;
mod notmuch;

use settings::some::Config as SomeConfig;
use settings::notmuch::Config as NotMuchConfig;


pub struct Settings{

    pub some_config: SomeConfig,
    pub notmuch_config: NotMuchConfig

}


impl Settings{

    pub fn new(location: &Path) -> Self {

        let some_conf = SomeConfig::load(location);

        let mut notmuch_config_path = PathBuf::from(&some_conf.notmuch.path);

        debug!("Loading notmuch config from {0:?}", notmuch_config_path);

        let notmuch_conf = NotMuchConfig::load(&notmuch_config_path);

        let settings = Settings {
            some_config: some_conf,
            notmuch_config: notmuch_conf
        };

        return settings;
    }

}
