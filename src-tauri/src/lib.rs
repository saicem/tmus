pub mod app;
pub mod cli;
pub mod cmd;
pub mod error;
pub mod state;
pub mod util;

use crate::app::constant::data_dir;
use crate::app::global::set_app_handle;
use crate::app::start_timer;
use crate::app::update;
pub use crate::cli::{Cli, LogLevel};
use crate::state::get_rule_radix_tree;
use tauri::{AppHandle, Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;
use tmus_engine::{async_runtime, engine_start, EngineMode};
use tokio::sync::Mutex;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;

pub fn setup(app: &mut tauri::App, nw: bool) -> Result<(), Box<dyn std::error::Error>> {
    engine_start(data_dir(), EngineMode::Working, |app_path| {
        get_rule_radix_tree().lock().unwrap().filter(app_path)
    })?;

    start_timer();

    app.manage(update::PendingUpdate(Mutex::new(None)));
    let app_handle = app.app_handle().clone();
    app::tray::tray(&app_handle).expect("Error while initializing tray");

    if !nw {
        app::focus_main_window(&app_handle);
    }

    set_app_handle(app_handle);

    Ok(())
}

pub fn event_callback(_: &AppHandle, event: RunEvent) {
    if let RunEvent::ExitRequested { api, .. } = event {
        api.prevent_exit();
    }
}

pub fn init_tracing(level: LogLevel) {
    let subscriber = FmtSubscriber::builder()
        .with_span_events(FmtSpan::ACTIVE)
        .with_max_level(Level::from(level))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}

pub async fn run_cli_mode(cli: Cli) {
    cli::run(cli).await;
}

#[tokio::main]
pub async fn run_gui_mode(cli: Cli) {
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
        .setup(move |app| setup(app, cli.nw))
        .invoke_handler(cmd::handler())
        .build(tauri::generate_context!())
        .expect("Error while building application");
    info!("Application started");
    app.run(event_callback);
}
