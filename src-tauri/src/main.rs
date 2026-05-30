// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::constant::data_dir;
use crate::app::global::set_app_handle;
use crate::app::start_timer;
use crate::app::update;
use crate::mcp::server::start_mcp_server;
use crate::state::{get_config, get_rule_radix_tree};
use std::env;
use tauri::{AppHandle, Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;
use tmus_engine::{async_runtime, engine_start};
use tokio::sync::Mutex;
use tracing::Level;
use tracing::{error, info};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;

mod app;
mod cmd;
mod mcp;
mod state;
mod util;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_span_events(FmtSpan::ACTIVE)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    util::force_singleton().await;
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    async_runtime::set_runtime(tokio::runtime::Handle::current());
    util::run_new_instance_listener().expect("Create new instance listener failed.");

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
        .invoke_handler(cmd::handler())
        .build(tauri::generate_context!())
        .expect("Error while building application");
    info!("Application started");
    app.run(event_callback);
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let config = { get_config().clone() };
    engine_start(data_dir(), |app_path| {
        get_rule_radix_tree().lock().unwrap().filter(app_path)
    });

    start_timer();

    if config.auto_start_mcp_server {
        let port = config.mcp_server_port;
        tauri::async_runtime::spawn(async move {
            match start_mcp_server(port).await {
                Ok(_) => info!("MCP server started on port {}", port),
                Err(e) => error!("Failed to start MCP server on port {}: {}", port, e),
            }
        });
    }

    app.manage(update::PendingUpdate(Mutex::new(None)));
    let app_handle = app.app_handle().clone();
    app::tray::tray(&app_handle).expect("Error while initializing tray");
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
