use std::fs::File;
use std::io::prelude::*;

use std::path::Path;
use std::collections::BTreeMap;
use toml;
use serde;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {

    #[serde(default = "default_version")]
    pub version: i16,

    #[serde(default)]
    pub debug: DebugConfig,

    #[serde(default)]
    pub notmuch: NotMuchConfig,

    #[serde(default)]
    pub accounts: BTreeMap<String, AccountConfig>,
    //shortcuts: ShortcutConfig,

}

impl Config{
    #[serde(skip_serializing)]
    pub fn load(location: &Path) -> Self {
        let mut conf_contents = String::new();

        match File::open(&location) {
            Ok(mut file) => {
                file.read_to_string(&mut conf_contents);
            },
            Err(err) => {
                conf_contents = "".to_string();
            },
        };


        let mut conf: Config = toml::from_str(&conf_contents).unwrap();

        return conf;
    }

    // #[serde(skip_serializing)]
    // pub fn store(self: &Self, location: &Path) -> Result<(), String> {
    //     let mut outfile = File::create(location).unwrap();
    //     outfile.write_all(toml::to_string(&self).unwrap().as_bytes());
    //     outfile.sync_all();
    //
    //     return Ok(());
    //
    // }

}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DebugConfig {
    #[serde(default = "default_debug_dryrun_sending")]
    pub dryrun_sending: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        DebugConfig{
            dryrun_sending: default_debug_dryrun_sending()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NotMuchConfig {
    #[serde(default = "default_notmuch_config_path")]
    pub path: String,
}


impl Default for NotMuchConfig {
    fn default() -> Self {
        NotMuchConfig{
            path: default_notmuch_config_path()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AccountConfig {
    pub default: bool,
    pub name: String,
    pub email: String,
}


fn default_version() -> i16 {
    return 1;
}

fn default_notmuch_config_path() -> String {
    return "~/.notmuch-config".to_string();
}

fn default_debug_dryrun_sending() -> bool {
    return false;
}
