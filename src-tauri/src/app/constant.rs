use std::{fs, path::PathBuf, sync::OnceLock};

use super::config::Config;

pub static DATA_DIR: OnceLock<String> = OnceLock::new();
pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init() {
    let data_dir = dirs_next::data_dir()
        .map(|dir| dir.join(&"tmus"))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    init_data_dir(&data_dir);
    DATA_DIR.set(data_dir.clone()).unwrap();
    CONFIG.set(Config::load(&data_dir)).unwrap();
}

fn init_data_dir(data_dir: &str) {
    let data_dir = PathBuf::from(data_dir);
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir.clone()).expect("create date directory failed.");
    }
}
