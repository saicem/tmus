use crate::app;
use std::io;
use std::process::exit;
use tauri::async_runtime::JoinHandle;
use tokio::net::windows::named_pipe::{ClientOptions, ServerOptions};
use tracing::{error, info};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::CreateMutexW,
    },
};

const MUTEX_KEY: &str = "tmus_singleton";
const PIPE_NAME: &str = r"\\.\pipe\tmus_singleton";

/// Check if the application has another instance which is already started.
pub async fn force_singleton() {
    let mutex = unsafe { CreateMutexW(None, true, &HSTRING::from(MUTEX_KEY)) };
    let last_error = unsafe { GetLastError() };

    if last_error == ERROR_ALREADY_EXISTS {
        info!("Another instance is already running.");
        run_client()
            .await
            .expect("Connect to pipeline server failed.");
        exit(0);
    }

    if mutex.is_err() {
        error!("Failed to create mutex, {}", mutex.err().unwrap());
        exit(1);
    }

    run_server().expect("Create pipeline server failed.");
}

fn focus_main_window() {
    app::focus_main_window(app::global::get_app_handle());
}

fn run_server() -> Result<(), io::Error> {
    let mut server = ServerOptions::new()
        .first_pipe_instance(true)
        .create(PIPE_NAME)?;

    let _server_task: JoinHandle<Result<(), io::Error>> = tauri::async_runtime::spawn(async move {
        loop {
            server.connect().await?;
            info!("Another instance is trying start.");
            focus_main_window();
            let _connected_client = server;
            server = ServerOptions::new().create(PIPE_NAME)?;
            let _client = tauri::async_runtime::spawn(async move {});
        }
    });
    Ok(())
}

async fn run_client() -> Result<(), io::Error> {
    ClientOptions::new().open(PIPE_NAME)?;
    Ok(())
}
