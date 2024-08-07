use crate::app::window::focus_main_window;
use tauri::AppHandle;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::SystemTrayMenuItem;
use tauri::SystemTraySubmenu;

pub fn menu() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_submenu(SystemTraySubmenu::new(
                "Language",
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("lang_en", "English"))
                    .add_item(CustomMenuItem::new("lang_zhs", "简体中文"))
                    .add_item(CustomMenuItem::new("lang_zht", "繁體中文")),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_submenu(SystemTraySubmenu::new(
                "Theme",
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("theme_light", "Light"))
                    .add_item(CustomMenuItem::new("theme_dark", "Dark")),
            ))
            .add_item(CustomMenuItem::new("quit", "Quit")),
    )
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            app.emit_all("menuItemClick", id.to_string()).unwrap();
            menu_item_click(&app, id);
        }
        SystemTrayEvent::LeftClick { .. } => {
            focus_main_window(&app);
        }
        _ => {}
    }
}

fn menu_item_click(app: &AppHandle, id: String) {
    match id.as_str() {
        lang if lang.starts_with("lang_") => {
            chose_one(app, &id, ["lang_en", "lang_zhs", "lang_zht"]);
        }
        theme if theme.starts_with("theme_") => {
            chose_one(app, &id, ["theme_light", "theme_dark"]);
        }
        "quit" => {
            std::process::exit(0);
        }
        _ => {}
    }
}

fn chose_one<const N: usize>(app: &AppHandle, chosen_id: &str, ids: [&str; N]) {
    ids.into_iter().for_each(|id| {
        let handle = app.tray_handle().get_item(id);
        handle.set_selected(id == chosen_id).unwrap();
    })
}
