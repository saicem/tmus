use crate::config::rule::Rule;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub lang: LangConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    pub rule: Rule,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum LangConfig {
    Zh,
    En,
    #[default]
    #[serde(other)]
    System,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ThemeConfig {
    Dark,
    Light,
    #[default]
    #[serde(other)]
    System,
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

}
