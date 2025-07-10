#![allow(dead_code)]

use crate::cmd::duration::query_duration_statistic;
use chrono::{DateTime, Local, Utc};
use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router, Error as McpError, RoleServer, ServerHandler,
};
use serde::{Deserialize, Serialize};
use std::future::Future;
use tmus_engine::util::Timestamp;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct StructRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Clone)]
pub struct Server {
    tool_router: ToolRouter<Server>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DurationStatisticQuery {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    merge_apps: bool,
    // app_ids: Option<HashSet<AppId>>,
    granularity: Timestamp,
    // cycle: Option<i64>,
}

#[tool_router]
impl Server {
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
    * `app_ids` - An optional list of application IDs to filter by.
      If only specific applications are of interest, provide their IDs here.
    * `granularity` - The time interval (in milliseconds) used to split records for aggregation.
      For example:
      - Use `86400000` to aggregate by day.
      - Use `3600000` to aggregate by hour.
    * `cycle` - An optional cycle length in units of `granularity`.
      This allows grouping intervals into repeating cycles.
      For example:
      - To analyze how much time is spent each hour within a day, use `granularity=3600000` and `cycle=24`.
      - To analyze how much time is spent each day within a week, use `granularity=86400000` and `cycle=7`."
    )]
    async fn query_duration_statistic(
        &self,
        Parameters(mut payload): Parameters<DurationStatisticQuery>,
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
                    // payload.app_ids,
                    payload.granularity,
                    None,
                    // payload.cycle,
                )
                .await,
            )
            .unwrap(),
        )]))
    }

    #[tool(description = "Get the current system local time and output it in ISO-8601 format.")]
    async fn get_current_time(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            Local::now().to_rfc3339(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for Server {
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
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}
