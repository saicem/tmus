use tauri::AppHandle;
use tauri::Manager;

pub fn focus_main_window(app: &AppHandle) {
    let main_window = app.get_webview_window("main");
    if main_window.is_none() {
        let window = tauri::WebviewWindowBuilder::from_config(
            app,
            &app.config().app.windows.get(0).unwrap().clone(),
        )
        .unwrap()
        .build()
        .unwrap();
    } else {
        main_window.unwrap().set_focus().unwrap();
    }
}
