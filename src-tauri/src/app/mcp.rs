use crate::app::mcp_service::McpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::tower::{
    StreamableHttpServerConfig, StreamableHttpService,
};
use serde::Serialize;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpServerState {
    port: u16,
    #[serde(skip)]
    ct: CancellationToken,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpServerStatus {
    running: bool,
    port: Option<u16>,
}

static MCP_SERVER_STATE: OnceLock<Mutex<Option<McpServerState>>> = OnceLock::new();

#[tauri::command]
#[tracing::instrument]
pub async fn start_mcp_server(port: u16) -> Result<(), String> {
    let Ok(mut server_state) = MCP_SERVER_STATE.get_or_init(|| Mutex::new(None)).try_lock() else {
        return Err("Another command is running".to_string());
    };
    if server_state.is_some() {
        return Err("Server is running".to_string());
    }

    let ct = CancellationToken::new();

    let service = StreamableHttpService::new(
        || Ok(McpService::new()),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default().with_cancellation_token(ct.clone()),
    );

    let router = axum::Router::new().nest_service("/mcp", service);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Server bind address failed");

    let shutdown_ct = ct.clone();
    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        shutdown_ct.cancelled().await;
        info!("Server cancelled");
    });

    tokio::spawn(async move {
        if let Err(e) = server.await {
            error!(error = %e, "Server shutdown with error");
        }
    });

    *server_state = Some(McpServerState { port, ct });
    info!("Server started");
    Ok(())
}

#[tauri::command]
#[tracing::instrument]
pub async fn stop_mcp_server() {
    let mut server_state_opt = MCP_SERVER_STATE
        .get_or_init(|| Mutex::new(None))
        .lock()
        .await;
    let Some(state) = server_state_opt.as_ref() else {
        info!("Server is not running");
        return;
    };
    info!("Stopping server...");
    state.ct.cancel();
    *server_state_opt = None;
}

#[tauri::command]
#[tracing::instrument]
pub async fn get_mcp_server_status() -> McpServerStatus {
    let state_opt = MCP_SERVER_STATE
        .get_or_init(|| Mutex::new(None))
        .lock()
        .await;
    if let Some(state) = state_opt.as_ref() {
        McpServerStatus {
            running: true,
            port: Some(state.port),
        }
    } else {
        McpServerStatus {
            running: false,
            port: None,
        }
    }
}
