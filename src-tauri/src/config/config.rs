use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub lang: LangConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default = "default_filter_uninstalled_app")]
    pub filter_uninstalled_app: bool,
    #[serde(default)]
    pub first_day_of_week: u8,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default = "default_time_format")]
    pub time_format: String,
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

fn default_filter_uninstalled_app() -> bool {
    true
}

fn default_date_format() -> String {
    "yyyy-MM-dd".to_string()
}

fn default_time_format() -> String {
    "H:mm:ss".to_string()
}
