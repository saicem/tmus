use crate::app::constant::{config_file_path, data_dir};
use crate::config::Config;
use std::fs;
use std::path::PathBuf;

pub(crate) fn init_data_dir() {
    let data_dir = PathBuf::from(data_dir());
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir.clone()).expect("create date directory failed.");
    }
}

pub(crate) fn init_config() {
    let config = Config::load(config_file_path());
    log::debug!("config: {:#?}", config);
    Config::set(config);
    log::debug!("config: {:#?}", Config::get_mut());
}
