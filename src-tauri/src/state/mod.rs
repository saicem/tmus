use crate::app::constant::{config_file_path, rule_file_path};
use crate::app::refresh_tray_menu;
use crate::state::config::Config;
use crate::util::dump_json;

mod config;
mod i18n;
mod radix;
mod rule;

pub use config::*;
pub use i18n::I18n;
pub use rule::get_rule;
pub use rule::get_rule_radix_tree;

#[tauri::command]
pub async fn get_app_config() -> Config {
    get_config().lock().unwrap().clone()
}

#[tauri::command]
pub async fn set_app_config(config: Config, app_handle: tauri::AppHandle) {
    dump_json(&config, config_file_path());
    {
        *get_config().lock().unwrap() = config
    };
    refresh_tray_menu(&app_handle);
}

#[tauri::command]
pub async fn get_app_rule() -> rule::Rule {
    get_rule().lock().unwrap().clone()
}

#[tauri::command]
pub async fn set_app_rule(config: rule::Rule) {
    dump_json(&config, rule_file_path());
    {
        *get_rule().lock().unwrap() = config.clone()
    };
}
