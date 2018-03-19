use std::fs::File;
use std::io::prelude::*;

use std::path::Path;
use std::collections::BTreeMap;
use toml;
use serde;

use serde_ini as ini;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {


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


        let mut conf: Config = ini::from_str(&conf_contents).unwrap();

        return conf;
    }

    #[serde(skip_serializing)]
    pub fn store(self: &Self, location: &Path) -> Result<(), String> {
        // let mut outfile = File::create(location).unwrap();
        // outfile.write_all(toml::to_string(&self).unwrap().as_bytes());
        // outfile.sync_all();

        return Ok(());

    }

}
