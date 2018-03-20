use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};

use std::collections::BTreeMap;
use toml;
use serde;

use settings::some::Config as SomeConfig;
use settings::notmuch::Config as NotMuchConfig;

pub struct SettingsManager {

    some_config: SomeConfig,
    notmuch_config: NotMuchConfig

}


impl SettingsManager{

    pub fn new(location: &Path) -> Self {

        let some_conf = SomeConfig::load(location);

        let mut notmuch_config_path = PathBuf::from(&some_conf.notmuch.path);

        debug!("Loading notmuch config from {0:?}", notmuch_config_path);

        let notmuch_conf = NotMuchConfig::load(&notmuch_config_path);

        let settings = SettingsManager {
            some_config: some_conf,
            notmuch_config: notmuch_conf
        };

        return settings;
    }

}
