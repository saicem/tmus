use std::path::PathBuf;
use once_cell::sync::OnceCell;

pub static DATA_DIR: OnceCell<PathBuf> = OnceCell::new();