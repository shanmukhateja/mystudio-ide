use std::{
    fs::File,
    io::BufReader,
    io::{BufWriter, Read, Write},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::fs::get_config_file_path;

#[derive(Serialize, Deserialize)]
pub struct AppConfigGeneralOptions {
    pub application_width: i32,
    pub application_height: i32,
}

impl Default for AppConfigGeneralOptions {
    fn default() -> Self {
        Self {
            application_width: 1024,
            application_height: 768,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub General: AppConfigGeneralOptions,
}

impl FromStr for AppConfig {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str::<AppConfig>(s)
    }
}

impl ToString for AppConfig {
    fn to_string(&self) -> String {
        toml::to_string(self).expect("Unable to serialize config.")
    }
}

pub trait AppConfigProvider {
    fn get_config() -> AppConfig;

    fn save(config: &AppConfig);
}

/**
The default implementation for storing/retrieving AppConfig.

It uses `.config/mystudio-ide` on Linux and `%USER%\AppData\Roaming\mystudio-ide` for Windows.
*/
pub struct DefaultAppConfigProvider;

impl AppConfigProvider for DefaultAppConfigProvider {
    fn get_config() -> AppConfig {
        let path_buf = get_config_file_path();

        // Create a config file with app defaults
        if !path_buf.exists() {
            let default_config = AppConfig::default();
            DefaultAppConfigProvider::save(&default_config);

            default_config
        } else {
            let file = File::open(path_buf).unwrap();

            let mut file_contents = String::new();

            let mut reader = BufReader::new(file);
            reader
                .read_to_string(&mut file_contents)
                .expect("Unable to read config file contents.");

            AppConfig::from_str(&file_contents).expect("Unable to parse config file contents.")
        }
    }

    fn save(config: &AppConfig) {
        let config_str = config.to_string();

        let path_buf = get_config_file_path();

        let mut writer = BufWriter::new(File::create(path_buf).unwrap());

        writer
            .write_all("# Config file documentation URL STUB\n\n".as_bytes())
            .unwrap();
        writer.write_all(config_str.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
