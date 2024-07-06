use std::sync::OnceLock;

pub static DEFAULT_DATA_DIR: OnceLock<String> = OnceLock::new();
