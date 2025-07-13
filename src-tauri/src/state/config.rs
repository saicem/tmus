use crate::app::constant::config_file_path;
use crate::util::load_json;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub lang: LangConfig,
    pub theme: ThemeConfig,
    pub filter_uninstalled_app: bool,
    pub first_day_of_week: u8,
    pub date_format: String,
    pub time_format: String,
    pub auto_check_update: bool,
    pub auto_start_mcp_server: bool,
    pub mcp_server_port: u16,
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

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: LangConfig::System,
            theme: ThemeConfig::System,
            filter_uninstalled_app: true,
            first_day_of_week: 0,
            date_format: "yyyy-MM-dd".to_string(),
            time_format: "H:mm:ss".to_string(),
            auto_check_update: true,
            auto_start_mcp_server: false,
            mcp_server_port: 2371u16,
        }
    }
}

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

pub fn get_config() -> &'static Mutex<Config> {
    CONFIG.get_or_init(|| Mutex::new(load_json(config_file_path())))
}
