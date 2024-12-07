// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, RunEvent};

mod app;
mod cmd;
mod engine;
mod util;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(app::setup)
        .invoke_handler(tauri::generate_handler![
            cmd::file_detail,
            cmd::duration_aggregate,
            cmd::duration_by_day,
            cmd::raw_record,
            cmd::read_reverse,
        ])
        .build(tauri::generate_context!())
        .expect("Error while building application");
    log::info!("Application started");
    app.run(event_callback);
}

fn event_callback(_: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}
