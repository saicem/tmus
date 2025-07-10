use crate::app::mcp_service::Server;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};

const BIND_ADDRESS: &str = "127.0.0.1:3000";

pub async fn start_mcp_service() -> anyhow::Result<()> {
    let config = SseServerConfig {
        bind: BIND_ADDRESS.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };

    let (sse_server, router) = SseServer::new(config);

    let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;

    let ct = sse_server.config.ct.child_token();

    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        ct.cancelled().await;
        tracing::info!("sse server cancelled");
    });

    tokio::spawn(async move {
        if let Err(e) = server.await {
            tracing::error!(error = %e, "sse server shutdown with error");
        }
    });

    let ct = sse_server.with_service(Server::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
