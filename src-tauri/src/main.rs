// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::tray;
use app::window::init_window_style;
use app::{constant, singleton::force_singleton};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, RunEvent};

mod app;
mod cmd;
mod engine;

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![
            cmd::file_detail,
            cmd::duration_aggregate,
            cmd::duration_by_day,
            cmd::raw_record,
            cmd::read_reverse,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(event_callback)
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    init_data_dir(app);
    force_singleton();
    init_window_style(&app.get_window("main").unwrap());
    engine::init(&PathBuf::from(
        constant::DEFAULT_DATA_DIR.get().unwrap().to_owned(),
    ));
    Ok(())
}

fn init_data_dir(app: &mut tauri::App) {
    let data_dir = app.handle().path_resolver().app_data_dir().unwrap();
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir.clone()).expect("create date directory failed.");
    }
    constant::DEFAULT_DATA_DIR
        .set(data_dir.to_str().unwrap().to_owned())
        .unwrap();
}

fn event_callback(_: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}
