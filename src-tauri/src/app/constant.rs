use std::sync::OnceLock;

pub static APP_NAME: &str = "tmus";

static CONFIG_FILE_NAME: &str = "config.json";

pub fn data_dir() -> &'static str {
    static DATA_DIR: OnceLock<String> = OnceLock::new();
    DATA_DIR.get_or_init(|| {
        dirs_next::data_dir()
            .unwrap()
            .join(&APP_NAME)
            .to_str()
            .unwrap()
            .to_string()
    })
}

pub fn config_file_path() -> &'static str {
    static CONFIG_FILE_PATH: OnceLock<String> = OnceLock::new();
    CONFIG_FILE_PATH.get_or_init(|| data_dir().to_owned() + "/" + CONFIG_FILE_NAME)
}
