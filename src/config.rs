use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

const CONFIG_NAME: &str = "./configuration.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub input_folder: String,
    pub output_folder: String,
    pub reorder_after_sort: bool,
}

impl Config {
    pub fn load() -> Option<Self> {
        //let config_dir = config_dir().expect("Couldn't get users config dir");
        let path = Path::new(CONFIG_NAME);

        if !path.exists() {
            return None;
        }

        let file_content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Couldn't read configuration file at {:?}", &path));

        let config: Self = toml::from_str(&file_content).unwrap_or_else(|err| {
                panic!(
                    "Couldn't parse config at {:?}. Reason: {}",
                    &path,
                    err.message()
                )
            });

        Some(config)
    }

    pub fn save(&self) {
        let path = Path::new(CONFIG_NAME);

        if let Err(err) = fs::write(&path, toml::to_string(&self).unwrap()) {
            panic!(
                "Couldn't write config to {:?}. Reason: {}",
                &path, err
            );
        }
    }
}



