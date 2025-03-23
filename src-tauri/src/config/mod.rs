use rule::Rule;

use crate::app::constant::{config_file_path, rule_file_path, tag_file_path};
use crate::app::refresh_tray_menu;
use crate::config::config::Config;
use crate::config::loader::Loadable;

pub mod config;
pub mod i18n;
mod loader;
mod radix;
pub mod rule;
pub mod tag;

pub static CONFIG: Loadable<Config> = Loadable::new();
pub static RULE: Loadable<rule::Rule> = Loadable::new();
pub static TAG: Loadable<tag::AppTag> = Loadable::new();

pub fn init() {
    CONFIG.load(config_file_path());
    RULE.load(rule_file_path());
    TAG.load(tag_file_path());
}

#[tauri::command]
pub async fn get_app_config() -> Config {
    CONFIG.get()
}

#[tauri::command]
pub async fn set_app_config(config: Config, app_handle: tauri::AppHandle) {
    CONFIG.set(&config);
    CONFIG.dump(config_file_path());
    refresh_tray_menu(&app_handle);
}

#[tauri::command]
pub async fn get_app_rule() -> rule::Rule {
    RULE.get()
}

#[tauri::command]
pub async fn set_app_rule(rule: Rule) {
    RULE.set(&rule);
    RULE.dump(rule_file_path());
}

#[tauri::command]
pub async fn get_app_tag() -> tag::AppTag {
    TAG.get()
}

#[tauri::command]
pub async fn set_app_tag(tag: tag::AppTag) {
    TAG.set(&tag);
    TAG.dump(tag_file_path());
}