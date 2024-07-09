use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Read,
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub lang: String,
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: "lang_zhs".to_string(),
            theme: "theme_light".to_string(),
        }
    }
}

impl Config {
    pub fn load(data_dir: &str) -> Config {
        let config_path = Self::config_path(data_dir);
        println!("config_path: {:?}", config_path.clone());
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(config_path)
            .expect("Read config.json fail");
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_default()
    }

    pub fn dump(&self, data_dir: &str) {
        let str = serde_json::to_string_pretty(self).unwrap();
        fs::write(Self::config_path(data_dir), str).unwrap()
    }

    fn config_path(data_dir: &str) -> PathBuf {
        PathBuf::from(data_dir).join("config.json")
    }
}
