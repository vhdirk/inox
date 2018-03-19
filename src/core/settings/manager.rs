use std::fs::File;
use std::io::prelude::*;

use std::path::Path;
use std::collections::BTreeMap;
use toml;
use serde;

use settings::some::Config;

pub struct SettingsManager {

    some_config: Config,
    // notmuch_config:

}


impl SettingsManager{

    pub fn new(location: &Path) -> Self {

        let some_conf = Config::load(location);


        let settings = SettingsManager {
            some_config: some_conf
        };

        return settings;
    }

}
