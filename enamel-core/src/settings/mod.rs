use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};

use std::collections::BTreeMap;
use toml;
use serde;

mod enamel;
mod notmuch;

use crate::settings::enamel::Config as EnamelConfig;
use crate::settings::notmuch::Config as NotMuchConfig;


#[derive(Clone, Debug)]
pub struct Settings{

    /// Path where config was loaded from
    pub config_path: PathBuf,

    pub enamel_config: EnamelConfig,
    pub notmuch_config: NotMuchConfig

}


impl Settings{

    pub fn new(location: &Path) -> Self {

        let enamel_conf = EnamelConfig::load(location);

        let notmuch_config_path = PathBuf::from(&enamel_conf.notmuch.path);

        debug!("Loading notmuch config from {0:?}", notmuch_config_path);

        let notmuch_conf = NotMuchConfig::load(&notmuch_config_path);

        let settings = Settings {
            config_path: location.into(),
            enamel_config: enamel_conf,
            notmuch_config: notmuch_conf
        };

        return settings;
    }

}
