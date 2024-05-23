use tauri::AppHandle;
use tauri::Manager;
use tauri::Window;

pub fn focus_main_window(app: &AppHandle) {
    let main_window = app.get_window("main");
    if main_window.is_none() {
        let window =
            tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
                .inner_size(900_f64, 600_f64)
                .build()
                .unwrap();
        init_window_style(&window);
    } else {
        main_window.unwrap().set_focus().unwrap();
    }
}

pub fn init_window_style(window: &Window) {
    window.set_decorations(false).unwrap();
    window_shadows::set_shadow(&window, true).unwrap()
}
