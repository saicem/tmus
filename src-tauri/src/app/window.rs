use crate::app::constant::APP_TITLE;
use tauri::AppHandle;
use tauri::Manager;

pub fn focus_main_window(app: &AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.set_focus().unwrap();
        return;
    }
    let _window = tauri::WebviewWindowBuilder::from_config(
        app,
        &app.config().app.windows.first().unwrap().clone(),
    )
    .unwrap()
    .title(APP_TITLE)
    .build()
    .unwrap();
    _window.set_focus().unwrap();
}
