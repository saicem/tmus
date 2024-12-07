use crate::engine::data::Millisecond;
use std::sync::OnceLock;

pub static APP_NAME: &str = "tmus";

pub static CONFIG_FILE_NAME: &str = "config.json";

pub static THRESHOLD: Millisecond = Millisecond::from_secs(1);

pub fn data_dir() -> &'static str {
    static DATA_DIR: OnceLock<String> = OnceLock::new();
    DATA_DIR.get_or_init(|| {
        dirs_next::data_dir()
            .map(|dir| dir.join(&APP_NAME))
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    })
}
