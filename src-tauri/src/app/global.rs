use std::sync::OnceLock;
use tauri::AppHandle;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn get_app_handle<'a>() -> &'a AppHandle {
    APP_HANDLE.get().unwrap()
}

pub fn set_app_handle(app_handle: AppHandle) {
    APP_HANDLE.set(app_handle).unwrap();
}
