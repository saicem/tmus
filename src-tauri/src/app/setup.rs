use crate::app::constant::{config_file_path, data_dir};
use crate::app::tray::tray;
use crate::config::Config;
use crate::{app, engine};
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
    let config = Config::load(config_file_path());
    log::debug!("config: {:#?}", config);
    Config::set(config);
    log::debug!("config: {:#?}", Config::get_mut());
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle();
    app::global::APP_HANDLE.set(app_handle.to_owned()).unwrap();
    init_data_dir();
    init_config();
    tray(app_handle).expect("Error while initializing tray");
    engine::init(&PathBuf::from(data_dir()));
    Ok(())
}
