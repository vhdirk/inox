use std::path::{Path, PathBuf};
use log::*;

mod inox;
mod notmuch;

use crate::settings::inox::Config as InoxConfig;
use crate::settings::notmuch::Config as NotMuchConfig;


#[derive(Clone, Debug)]
pub struct Settings{

    /// Path where config was loaded from
    pub config_path: PathBuf,

    pub inox_config: InoxConfig,
    pub notmuch_config: NotMuchConfig

}


impl Settings{

    pub fn new(location: &Path) -> Self {

        let inox_conf = InoxConfig::load(location);

        let notmuch_config_path = PathBuf::from(&inox_conf.notmuch.path);

        debug!("Loading notmuch config from {0:?}", notmuch_config_path);

        let notmuch_conf = NotMuchConfig::load(&notmuch_config_path);

        Settings {
            config_path: location.into(),
            inox_config: inox_conf,
            notmuch_config: notmuch_conf
        }
    }
}
