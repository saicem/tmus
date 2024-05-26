use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::global;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Lang id, determine the language of the interface.
    lang: String,

    /// When logging the focus event, immediately flush if true or control by operate system if false.
    always_flush: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            lang: "lang_zhs".to_string(),
            always_flush: true,
        }
    }
}

impl Config {
    /// Load from file
    pub fn load(&self) -> Config {
        let str = fs::read_to_string(Self::file_path()).unwrap();
        serde_json::from_str(&str).unwrap()
    }

    /// Dump config to file
    pub fn dump(&self) {
        let str = serde_json::to_string_pretty(self).unwrap();
        fs::write(Self::file_path(), str).unwrap()
    }

    fn file_path<'a>() -> PathBuf {
        global::DATA_DIR.get().unwrap().join("config.json")
    }
}
