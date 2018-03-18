use std::fs::File;
use std::io::prelude::*;

use std::path::Path;
use std::collections::BTreeMap;
use toml;
use serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: i16,
    debug: DebugConfig,
    notmuch: NotMuchConfig,

    accounts: BTreeMap<String, AccountConfig>,
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
                conf_contents = DEFAULT_CONFIG.to_string();
            },
        };


        let mut conf: Config = toml::from_str(&conf_contents).unwrap();
        return conf;
    }

    #[serde(skip_serializing)]
    pub fn store(self: &Self, location: &Path) -> Result<(), String> {
        let mut outfile = File::create(location).unwrap();
        outfile.write_all(toml::to_string(&self).unwrap().as_bytes());
        outfile.sync_all();

        return Ok(());

    }

}



#[derive(Serialize, Deserialize, Debug)]
struct DebugConfig {
    dryrun_sending: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct NotMuchConfig {
    config: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountConfig {
    default: bool,
    name: String,
    email: String,


    // "name": "Dirk Van Haerenborgh",
    // "email": "dirk.vanhaerenborgh@senso2.me",
    // "gpgkey": "",
    // "always_gpg_sign": "false",
    // "sendmail": "msmtp --read-envelope-from -i -t",
    // "default": "true",
    // "save_sent": "false",
    // "save_sent_to": "\/home\/dvhaeren\/.mail\/sent\/cur\/",
    // "additional_sent_tags": "",
    // "save_drafts_to": "\/home\/root\/.mail\/drafts\/",
    // "signature_separate": "false",
    // "signature_file": "",
    // "signature_file_markdown": "",
    // "signature_default_on": "true",
    // "signature_attach": "false",
    // "select_query": ""
}



pub const DEFAULT_CONFIG: &'static str = "

# version of the config file (for parser)
version = 1

[debug]
dryrun_sending = false

[notmuch]
config =  \"~/.notmuch-config\"

[accounts]


";
