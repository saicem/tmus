pub mod app;
pub mod cli;
pub mod cmd;
pub mod mcp;
pub mod state;
pub mod util;

use crate::app::constant::data_dir;
use crate::app::global::set_app_handle;
use crate::app::start_timer;
use crate::app::update;
use crate::cli::handle_cli;
pub use crate::cli::{Cli, LogLevel};
use crate::mcp::server::start_mcp_server;
use crate::state::{get_config, get_rule_radix_tree};
use tauri::{AppHandle, Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;
use tmus_engine::{async_runtime, engine_start};
use tokio::sync::Mutex;
use tracing::Level;
use tracing::{error, info};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;

pub fn setup(app: &mut tauri::App, nw: bool) -> Result<(), Box<dyn std::error::Error>> {
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

    if !nw {
        app::focus_main_window(&app_handle);
    }

    set_app_handle(app_handle);

    Ok(())
}

pub fn event_callback(_: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}

pub fn init_tracing(level: Option<LogLevel>) {
    let subscriber = FmtSubscriber::builder()
        .with_span_events(FmtSpan::ACTIVE)
        .with_max_level(level.map(|l| Level::from(l)).unwrap_or(Level::INFO))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}

pub fn run_cli_mode(cli: Cli) {
    engine_start(data_dir(), |_app_path| Some(_app_path.to_string()));
    handle_cli(cli);
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
