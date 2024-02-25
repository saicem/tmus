// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use crate::app::file::core;
use crate::app::file_version::file_version;
use crate::app::monitor::set_event_hook;
use crate::app::tray;
use std::fs;
use tauri::{AppHandle, RunEvent};

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![file_version])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(event_callback)
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = app.handle().path_resolver().app_data_dir().unwrap();
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir.clone()).expect("create date directory failed.");
    }
    unsafe {
        core::init(&data_dir);
    }
    set_event_hook();
    Ok(())
}

fn event_callback(app_handle: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}
