use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub lang: LangConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
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
