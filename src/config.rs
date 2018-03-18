use std::collections::BTreeMap;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: i16,
    debug: DebugConfig,
    notmuch: NotMuchConfig,

    accounts: BTreeMap<String, AccountConfig>,

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
