use std::path::PathBuf;
use std::sync::OnceLock;

pub static APP_NAME: &str = "tmus";

pub fn data_dir() -> &'static PathBuf {
    static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();
    DATA_DIR.get_or_init(|| dirs_next::data_dir().unwrap().join(&APP_NAME))
}

pub fn cache_dir() -> &'static PathBuf {
    static CACHE_DIR: OnceLock<PathBuf> = OnceLock::new();
    CACHE_DIR.get_or_init(|| dirs_next::cache_dir().unwrap().join(&APP_NAME))
}

pub fn config_file_path() -> &'static PathBuf {
    static CONFIG_FILE_PATH: OnceLock<PathBuf> = OnceLock::new();
    CONFIG_FILE_PATH.get_or_init(|| data_dir().join("config.json"))
}

pub fn rule_file_path() -> &'static PathBuf {
    static RULE_FILE_PATH: OnceLock<PathBuf> = OnceLock::new();
    RULE_FILE_PATH.get_or_init(|| data_dir().join("rule.json"))
}

pub fn app_detail_cache_path() -> &'static PathBuf {
    static APP_DETAIL_CACHE_PATH: OnceLock<PathBuf> = OnceLock::new();
    APP_DETAIL_CACHE_PATH.get_or_init(|| cache_dir().join("app_detail.json"))
}
