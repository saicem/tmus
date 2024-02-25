use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            "Language",
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("lang_en".to_string(), "English"))
                .add_item(CustomMenuItem::new("lang_zhs".to_string(), "简体中文"))
                .add_item(CustomMenuItem::new("lang_zht".to_string(), "繁體中文")),
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            lang if lang.starts_with("lang_") => {
                chose_one(app, id, ["lang_en", "lang_zhs", "lang_zht"])
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        SystemTrayEvent::LeftClick { .. } => {
            let main_window = app.get_window("main");
            if main_window.is_none() {
                let window = tauri::WindowBuilder::new(
                    app,
                    "main",
                    tauri::WindowUrl::App("index.html".into()),
                )
                .inner_size(900_f64, 600_f64)
                .build()
                .unwrap();
                window.set_decorations(false).unwrap();
            } else {
                main_window.unwrap().set_focus().unwrap();
            }
        }
        _ => {}
    }
}

fn chose_one<const N: usize>(app: &AppHandle, chosen_id: String, ids: [&str; N]) {
    ids.into_iter().for_each(|id| {
        let handle = app.tray_handle().get_item(id);
        handle.set_selected(id == chosen_id).unwrap();
    })
}
