use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum LangConfig {
    Zh,
    #[default]
    #[serde(other)]
    En,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ThemeConfig {
    Dark,
    Light,
    #[default]
    #[serde(other)]
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub lang: LangConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
}

impl Config {
    pub fn get_mut<'a>() -> MutexGuard<'a, Config> {
        static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();
        CONFIG
            .get_or_init(|| Mutex::new(Config::default()))
            .lock()
            .unwrap()
    }

    pub fn get() -> Config {
        Self::get_mut().clone()
    }

    pub fn set(config: Config) {
        *Self::get_mut() = config;
    }

    pub fn load<P: AsRef<Path>>(file_path: P) -> Config {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap()
    }

    pub fn dump<P: AsRef<Path>>(&self, file_path: P) {
        let str = serde_json::to_string_pretty(self).unwrap();
        fs::write(file_path, str).unwrap()
    }
}
