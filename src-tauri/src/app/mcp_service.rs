#![allow(dead_code)]

use crate::cmd::duration::query_duration_statistic;
use chrono::{DateTime, Local, Utc};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{
    handler::server::router::tool::ToolRouter, model::*, schemars, service::RequestContext, tool,
    tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use serde::{Deserialize, Serialize};
use tmus_engine::storage::focus_app;
use tmus_engine::util::Timestamp;

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub id: usize,
    pub path: String,
}

#[derive(Clone)]
pub struct McpService {
    tool_router: ToolRouter<McpService>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DurationStatisticQuery {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    merge_apps: bool,
    granularity: Timestamp,
}

#[tool_router]
impl McpService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Query duration statistics with customizable filters and aggregation.
    # Arguments

    * `start` - The start time (inclusive) for filtering focus records, use ISO 8601 format.
    * `end` - The end time (inclusive) for filtering focus records, use ISO 8601 format.
    * `merge_apps` - Whether to merge data across all applications into a single result set.
      Set this to `true` if application-specific details are not required.
    * `granularity` - The time interval (in milliseconds) used to split records for aggregation.
      For example:
      - Use `86400000` to aggregate by day.
      - Use `3600000` to aggregate by hour."
    )]
    async fn query_duration_statistic(
        &self,
        Parameters(payload): Parameters<DurationStatisticQuery>,
    ) -> Result<CallToolResult, McpError> {
        let start_ts = payload.start.timestamp_millis();
        let end_ts = payload.end.timestamp_millis();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(
                &query_duration_statistic(
                    start_ts,
                    end_ts,
                    payload.merge_apps,
                    None,
                    payload.granularity,
                    None,
                )
                .await,
            )
            .unwrap(),
        )]))
    }

    #[tool(
        description = "Get current system local time with timezone and output it in ISO-8601 format."
    )]
    async fn get_current_time(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            Local::now().to_rfc3339(),
        )]))
    }

    #[tool(description = "Get all app with `id` and `path`.\
    Id is same with the result of `query_duration_statistic`.\
    When you need get the exact app name from `query_duration_statistic`.\
    Match the id and the extract name from path.")]
    async fn get_all_app(&self) -> Result<CallToolResult, McpError> {
        let app_vec = focus_app::get_all_app();
        let result = app_vec
            .into_iter()
            .enumerate()
            .map(|(id, path)| App { id, path })
            .collect::<Vec<_>>();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&result).unwrap(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for McpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides a counter tool that can increment and decrement values. The counter starts at 0 and can be modified using the 'increment' and 'decrement' tools. Use 'get_value' to check the current count.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "Initialize from http server");
        }
        Ok(self.get_info())
    }
}
