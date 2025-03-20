// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::constant::data_dir;
use std::env;
use std::path::PathBuf;
use app::constant::rule_file_path;
use config::{config_loader::ConfigLoader, rule::Rule};
use tauri::{AppHandle, Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;

mod app;
mod cmd;
mod config;
mod engine;
mod util;

fn main() {
    tauri::async_runtime::block_on(util::force_singleton());
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["nw"]),
        ))
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            cmd::file_detail,
            cmd::duration_by_id,
            cmd::duration_by_day,
            cmd::duration_by_day_id,
            cmd::raw_record,
            cmd::get_app_config,
            cmd::set_app_config,
            cmd::show_in_folder,
        ])
        .build(tauri::generate_context!())
        .expect("Error while building application");
    log::info!("Application started");
    app.run(event_callback);
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle();
    app::global::APP_HANDLE.set(app_handle.to_owned()).unwrap();
    app::setup::init_data_dir();
    app::setup::init_config();
    app::tray::tray(app_handle).expect("Error while initializing tray");

    let rule = Rule::load(rule_file_path());
    config::rule::init(&rule);
    engine::init(&PathBuf::from(data_dir()), |app_path| {
        if config::rule::is_exclude(&app_path) {
            return None;
        }
        config::rule::get_merged_path(&app_path).map(|x|x.as_ref().to_string()).or(Some(app_path.to_owned()))
    });
    handle_start_args(app_handle);
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

fn handle_start_args(app_handle: &AppHandle) {
    let no_window = env::args().any(|x| x == "nw");
    if !no_window {
        app::focus_main_window(app_handle);
    }
}
