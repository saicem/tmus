// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::constant::data_dir;
use crate::config::rule::{is_exclude, is_include};
use crate::config::{get_app_config, set_app_config, RULE};
use config::{get_app_rule, get_app_tag, set_app_rule, set_app_tag};
use engine::{get_engine, Engine};
use log;
use std::path::PathBuf;
use std::{env, fs};
use tauri::{AppHandle, Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log;

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
            get_app_config,
            set_app_config,
            get_app_rule,
            set_app_rule,
            get_app_tag,
            set_app_tag,
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

    init_data_dir();

    config::init();
    config::rule::init_rule(&RULE.get());

    app::tray::tray(app_handle).expect("Error while initializing tray");
    

    let receiver = engine::init(&PathBuf::from(data_dir()));
    let app_vec = get_engine().get_all_app();

    engine::start(
        |app_path| {
            if app_path.is_empty() || (is_exclude(&app_path) && !is_include(&app_path)) {
                return None;
            }
            config::rule::get_merged_path(&app_path)
                .map(|x| x.as_ref().to_string())
                .or(Some(app_path.to_owned()))
        },
        receiver,
    );
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

pub(crate) fn init_data_dir() {
    let data_dir = PathBuf::from(data_dir());
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir.clone()).expect("create date directory failed.");
    }
}
