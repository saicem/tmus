use tauri::AppHandle;
use tauri::Manager;

pub fn focus_main_window(app: &AppHandle) {
    let main_window = app.get_webview_window("main");
    if main_window.is_none() {
        let window = tauri::WebviewWindowBuilder::new(
            app,
            "main",
            tauri::WebviewUrl::App("index.html".into()),
        )
        .inner_size(1200_f64, 680_f64)
        .build()
        .unwrap();
        window.set_decorations(false).unwrap();
    } else {
        main_window.unwrap().set_focus().unwrap();
    }
}
