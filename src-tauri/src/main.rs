// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use tauri::{AppHandle, Manager, RunEvent};

use crate::app::api::duration_aggregate;
use crate::app::api::duration_by_day;
use crate::app::api::file_detail;
use crate::app::monitor::set_event_hook;
use crate::app::tray;
use crate::app::window::init_window_style;
use crate::app::{global, persist};

mod app;

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![
            file_detail,
            duration_aggregate,
            duration_by_day
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(event_callback)
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    init_window_style(&app.get_window("main").unwrap());
    let data_dir = app.handle().path_resolver().app_data_dir().unwrap();
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir.clone()).expect("create date directory failed.");
    }
    global::DATA_DIR.set(data_dir).unwrap();
    persist::init();
    set_event_hook();
    Ok(())
}

fn event_callback(_: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}
