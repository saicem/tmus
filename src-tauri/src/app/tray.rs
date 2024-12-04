use crate::app::window::focus_main_window;
use std::error::Error;
use tauri::menu::{CheckMenuItem, Menu, MenuBuilder, MenuEvent, MenuItemBuilder, SubmenuBuilder};
use tauri::{AppHandle, Wry};

use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent, TrayIconId};

pub fn tray(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
    let menu = build_menu(
        app_handle,
        MenuConfig {
            lang: "lang_zh".into(),
            theme: "theme_light".into(),
        },
    )?;
    let tray_icon_id = TrayIconId::new("main");
    let tray = app_handle.tray_by_id(&tray_icon_id).unwrap();
    tray.set_menu(Some(menu))?;
    tray.on_tray_icon_event(on_tray_icon_event);
    tray.on_menu_event(on_menu_event);
    Ok(())
}

pub struct MenuConfig {
    lang: String,
    theme: String,
}

fn build_menu(app_handle: &AppHandle, config: MenuConfig) -> Result<Menu<Wry>, Box<dyn Error>> {
    let lang_menu = SubmenuBuilder::new(app_handle, "Language")
        .items(&[
            &CheckMenuItem::with_id(
                app_handle,
                "lang_en",
                "English",
                true,
                config.lang == "lang_en",
                None::<&str>,
            )?,
            &CheckMenuItem::with_id(
                app_handle,
                "lang_zh",
                "简体中文",
                true,
                config.lang == "lang_zh",
                None::<&str>,
            )?,
        ])
        .separator()
        .build()?;
    let theme_menu = SubmenuBuilder::new(app_handle, "Theme")
        .items(&[
            &CheckMenuItem::with_id(
                app_handle,
                "theme_light",
                "Light",
                true,
                config.theme == "theme_light",
                None::<&str>,
            )?,
            &CheckMenuItem::with_id(
                app_handle,
                "theme_dark",
                "Dark",
                true,
                config.theme == "theme_dark",
                None::<&str>,
            )?,
        ])
        .build()?;
    let menu = MenuBuilder::new(app_handle)
        .items(&[
            &lang_menu,
            &theme_menu,
            &MenuItemBuilder::with_id("quit", "Quit").build(app_handle)?,
        ])
        .build()?;
    Ok(menu)
}

fn on_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Down,
        ..
    } = event
    {
        focus_main_window(tray.app_handle())
    }
}

fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    // app.emit_all("menuItemClick", event.id.0).unwrap();
    match event.id().as_ref() {
        lang if lang.starts_with("lang_") => {}
        theme if theme.starts_with("theme_") => {}
        "quit" => {
            std::process::exit(0);
        }
        _ => {}
    }
    // update every time
    // app.tray_by_id("main")?.set_menu(build_menu());
}
