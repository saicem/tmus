// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::tray;
use app::window::init_window_style;
use app::{constant, singleton::force_singleton};
use env_logger::Builder;
use log::info;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, RunEvent};

mod app;
mod cmd;
mod engine;

fn main() {
    init_logger();
    info!("Tmus start");
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
    constant::init();
    force_singleton();
    init_window_style(&app.get_window("main").unwrap());
    engine::init(&PathBuf::from(constant::DATA_DIR.get().unwrap().to_owned()));
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

#[cfg(not(target_env = "production"))]
fn init_logger() {
    std::env::set_var("RUST_LOG", "debug");
    Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();
}

#[cfg(target_env = "production")]
fn init_logger() {
    let mut builder = Builder::from_default_env();
    builder.target(env_logger::Target::Pipe(()));
    todo!()
}
