use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};

use std::collections::BTreeMap;
use toml;
use serde;

mod inox;
mod notmuch;

use settings::inox::Config as InoxConfig;
use settings::notmuch::Config as NotMuchConfig;


pub struct Settings{

    pub inox_config: InoxConfig,
    pub notmuch_config: NotMuchConfig

}


impl Settings{

    pub fn new(location: &Path) -> Self {

        let inox_conf = InoxConfig::load(location);

        let mut notmuch_config_path = PathBuf::from(&inox_conf.notmuch.path);

        debug!("Loading notmuch config from {0:?}", notmuch_config_path);

        let notmuch_conf = NotMuchConfig::load(&notmuch_config_path);

        let settings = Settings {
            inox_config: inox_conf,
            notmuch_config: notmuch_conf
        };

        return settings;
    }

}
