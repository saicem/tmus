use super::constant;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::OnceLock};

static CONFIG: OnceLock<Config> = OnceLock::<Config>::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Lang id, determine the language of the interface.
    pub lang: String,

    /// When logging the focus event, immediately flush if true or control by operate system if false.
    pub always_flush: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: "lang_zhs".to_string(),
            always_flush: true,
        }
    }
}

impl Config {
    /// Load from file
    pub fn load(&self) -> Config {
        let str = fs::read_to_string(Self::config_path()).unwrap();
        serde_json::from_str(&str).unwrap()
    }

    /// Dump config to file
    pub fn dump(&self) {
        let str = serde_json::to_string_pretty(self).unwrap();
        fs::write(Self::config_path(), str).unwrap()
    }

    fn config_path() -> PathBuf {
        PathBuf::from(constant::DEFAULT_DATA_DIR.get().unwrap()).join("config.json")
    }
}
