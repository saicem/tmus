use crate::app::constant::config_file_path;
use crate::app::window::focus_main_window;
use crate::config::config::{LangConfig, ThemeConfig};
use crate::config::i18n::I18n;
use crate::config::CONFIG;
use std::error::Error;
use std::sync::LazyLock;
use tauri::menu::{CheckMenuItem, Menu, MenuBuilder, MenuEvent, MenuItemBuilder, SubmenuBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent, TrayIconId};
use tauri::{AppHandle, Emitter, Wry};

static TRAY_ICON_ID: LazyLock<TrayIconId> = LazyLock::new(|| TrayIconId::new("main"));

pub fn tray(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
    let menu = build_menu(app_handle)?;
    let tray = app_handle.tray_by_id(&*TRAY_ICON_ID).unwrap();
    tray.set_menu(Some(menu))?;
    tray.on_tray_icon_event(on_tray_icon_event);
    tray.on_menu_event(on_menu_event);
    Ok(())
}

pub fn refresh_tray_menu(app_handle: &AppHandle) {
    let menu = build_menu(app_handle).unwrap();
    app_handle
        .tray_by_id(&*TRAY_ICON_ID)
        .unwrap()
        .set_menu(Some(menu))
        .unwrap();
}

fn build_menu(app_handle: &AppHandle) -> Result<Menu<Wry>, Box<dyn Error>> {
    let config = CONFIG.get();
    let lang_menu = SubmenuBuilder::new(app_handle, I18n::get().language)
        .items(&[
            &CheckMenuItem::with_id(
                app_handle,
                "lang_system",
                I18n::get().language_system,
                true,
                config.lang == LangConfig::System,
                None::<&str>,
            )?,
            &CheckMenuItem::with_id(
                app_handle,
                "lang_en",
                "English",
                true,
                config.lang == LangConfig::En,
                None::<&str>,
            )?,
            &CheckMenuItem::with_id(
                app_handle,
                "lang_zh",
                "简体中文",
                true,
                config.lang == LangConfig::Zh,
                None::<&str>,
            )?,
        ])
        .build()?;
    let theme_menu = SubmenuBuilder::new(app_handle, I18n::get().theme)
        .items(&[
            &CheckMenuItem::with_id(
                app_handle,
                "theme_system",
                I18n::get().theme_system,
                true,
                config.theme == ThemeConfig::System,
                None::<&str>,
            )?,
            &CheckMenuItem::with_id(
                app_handle,
                "theme_light",
                I18n::get().theme_light,
                true,
                config.theme == ThemeConfig::Light,
                None::<&str>,
            )?,
            &CheckMenuItem::with_id(
                app_handle,
                "theme_dark",
                I18n::get().theme_dark,
                true,
                config.theme == ThemeConfig::Dark,
                None::<&str>,
            )?,
        ])
        .build()?;
    let menu = MenuBuilder::new(app_handle)
        .items(&[
            &lang_menu,
            &theme_menu,
            &MenuItemBuilder::with_id("quit", I18n::get().exit).build(app_handle)?,
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

fn on_menu_event(app_handle: &AppHandle, event: MenuEvent) {
    let event_id = event.id.as_ref();
    app_handle.emit("menuItemClick", event_id).unwrap();
    match event_id {
        "quit" => {
            std::process::exit(0);
        }
        _ => {
            match event_id {
                "lang_en" => {
                    CONFIG.set_field(|x| x.lang = LangConfig::En);
                }
                "lang_zh" => {
                    CONFIG.set_field(|x| x.lang = LangConfig::Zh);
                }
                "lang_system" => {
                    CONFIG.set_field(|x| x.lang = LangConfig::System);
                }
                "theme_system" => {
                    CONFIG.set_field(|x| x.theme = ThemeConfig::System);
                }
                "theme_light" => {
                    CONFIG.set_field(|x| x.theme = ThemeConfig::Light);
                }
                "theme_dark" => {
                    CONFIG.set_field(|x| x.theme = ThemeConfig::Dark);
                }
                _ => {}
            }
            CONFIG.dump(config_file_path());
            refresh_tray_menu(app_handle);
        }
    }
}
