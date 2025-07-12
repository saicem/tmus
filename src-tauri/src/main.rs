// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::constant::data_dir;
use crate::app::global::set_app_handle;
use crate::app::mcp::start_mcp_service;
use crate::app::update;
use crate::config::rule::{is_exclude, is_include};
use crate::config::{get_app_config, set_app_config, RULE};
use config::{get_app_rule, set_app_rule};
use std::env;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;
use tmus_engine::{async_runtime, engine_start};
use tokio::sync::Mutex;

mod app;
mod cmd;
mod config;
mod util;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    async_runtime::set_runtime(tokio::runtime::Handle::current());
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    util::force_singleton().await;
    tauri::async_runtime::spawn(start_mcp_service());
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["nw"]),
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            cmd::get_raw_record,
            get_app_config,
            set_app_config,
            get_app_rule,
            set_app_rule,
            cmd::show_in_folder,
            cmd::get_tmus_meta,
            cmd::focus_index_record,
            cmd::app_detail::get_app_detail,
            cmd::app_detail::get_all_app_detail,
            update::fetch_update,
            update::install_update,
            cmd::app_duration_area::get_app_duration_area,
            cmd::duration::get_duration_by_id,
            cmd::duration::query_duration_statistic
        ])
        .build(tauri::generate_context!())
        .expect("Error while building application");
    tracing::info!("Application started");
    app.run(event_callback);
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    app.manage(update::PendingUpdate(Mutex::new(None)));

    let app_handle = app.app_handle().clone();

    config::init();
    config::rule::init_rule(&RULE.get());
    app::tray::tray(&app_handle).expect("Error while initializing tray");

    engine_start(data_dir(), |app_path| {
        if app_path.is_empty() || (is_exclude(&app_path) && !is_include(&app_path)) {
            return None;
        }
        config::rule::get_merged_path(&app_path).or(Some(app_path.to_owned()))
    });
    handle_start_args(&app_handle);
    set_app_handle(app_handle);
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
