/// Global instance.
use std::sync::OnceLock;
use tauri::AppHandle;


pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
