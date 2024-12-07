use crate::app::config::Config;
use crate::app::constant;
use crate::app::constant::{data_dir, CONFIG_FILE_NAME};
use crate::app::tray::tray;
use crate::engine;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

fn init_data_dir() {
    let data_dir = PathBuf::from(data_dir());
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir.clone()).expect("create date directory failed.");
    }
}

fn init_config() {
    let config_path = PathBuf::from(data_dir()).join(CONFIG_FILE_NAME);
    let config = Config::load(config_path);
    log::debug!("config: {:#?}", config);
    Config::set(config);
    log::debug!("config: {:#?}", Config::get());
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle();
    init_data_dir();
    init_config();
    tray(app_handle).expect("Error while initializing tray");
    engine::init(&PathBuf::from(constant::data_dir()));
    Ok(())
}
