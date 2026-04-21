#![allow(dead_code)]

use crate::cmd::app_detail::get_all_app_detail;
use crate::cmd::category;
use crate::cmd::duration::get_duration_by_id;
use crate::state::category::CategoryId;
use chrono::{DateTime, Local, Utc};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{
    handler::server::router::tool::ToolRouter, model::*, schemars, service::RequestContext, tool,
    tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use serde::{Deserialize, Serialize};
use tmus_engine::models::AppId;

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub id: AppId,
    pub path: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppUsageCommand {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    top_k: Option<usize>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppCategoryCommand {
    app_id: AppId,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetAppCategoryCommand {
    app_id: AppId,
    category_id: CategoryId,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetAppCategoryCommand {
    app_ids: Vec<AppId>,
    category_id: CategoryId,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddCategoryCommand {
    parent_id: CategoryId,
    name: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUncategorizedAppsCommand {
    limit: Option<usize>,
}

#[derive(Clone)]
pub struct McpService {
    tool_router: ToolRouter<McpService>,
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
        description = "Get current system local time with timezone and output it in ISO-8601 format."
    )]
    async fn get_current_time(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            Local::now().to_rfc3339(),
        )]))
    }

    #[tool(
        description = "Query app usage duration in specified time range, sorted by duration in descending order.
    # Arguments

    * `start` - The start time (inclusive) for filtering focus records, use ISO 8601 format.
    * `end` - The end time (inclusive) for filtering focus records, use ISO 8601 format.
    * `top_k` - Optional limit to return only top N apps with highest duration."
    )]
    async fn query_app_usage(
        &self,
        Parameters(payload): Parameters<AppUsageCommand>,
    ) -> Result<CallToolResult, McpError> {
        let start_ts = payload.start.timestamp_millis();
        let end_ts = payload.end.timestamp_millis();

        // Get app durations
        let mut app_durations = get_duration_by_id(start_ts, end_ts);

        // Sort app durations by duration in descending order
        app_durations.sort_by(|a, b| b.duration.cmp(&a.duration));

        // Apply top_k if provided early
        if let Some(top_k) = payload.top_k {
            app_durations.truncate(top_k);
        }

        let app_detail_map = get_all_app_detail().await;

        // Combine app durations with app details
        let app_usage: Vec<serde_json::Value> = app_durations
            .into_iter()
            .filter_map(|id_duration| {
                // Directly access fields since they are public
                let app_id = id_duration.app_id;
                let duration = id_duration.duration;

                app_detail_map.get(&app_id).map(|detail| {
                    serde_json::json!({
                        "id": app_id,
                        "name": detail.name,
                        "duration": duration
                    })
                })
            })
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&app_usage).unwrap(),
        )]))
    }

    #[tool(description = "Get all categories in tree structure.")]
    async fn get_categories(&self) -> Result<CallToolResult, McpError> {
        let categories = category::get_categories();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&categories).unwrap(),
        )]))
    }

    #[tool(description = "Get category of specified app.
    # Arguments

    * `app_id` - The ID of the app to query.")]
    async fn get_app_category(
        &self,
        Parameters(payload): Parameters<AppCategoryCommand>,
    ) -> Result<CallToolResult, McpError> {
        let category = crate::state::category::get_app_category(payload.app_id);
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&category).unwrap(),
        )]))
    }

    #[tool(description = "Set category for specified app.
    # Arguments

    * `app_id` - The ID of the app to set category.
    * `category_id` - The ID of the category to assign to the app.")]
    async fn set_app_category(
        &self,
        Parameters(payload): Parameters<SetAppCategoryCommand>,
    ) -> Result<CallToolResult, McpError> {
        let _ = crate::state::category::set_app_category(payload.app_id, payload.category_id);
        Ok(CallToolResult::success(vec![Content::text(
            "{\"success\": true}",
        )]))
    }

    #[tool(description = "Batch set category for multiple apps.
    # Arguments

    * `app_ids` - The IDs of the apps to set category.
    * `category_id` - The ID of the category to assign to the apps.")]
    async fn batch_set_app_category(
        &self,
        Parameters(payload): Parameters<BatchSetAppCategoryCommand>,
    ) -> Result<CallToolResult, McpError> {
        for app_id in &payload.app_ids {
            let _ = crate::state::category::set_app_category(*app_id, payload.category_id);
        }
        Ok(CallToolResult::success(vec![Content::text(
            "{\"success\": true}",
        )]))
    }

    #[tool(description = "Add new category under specified parent category.
    # Arguments

    * `parent_id` - The ID of the parent category.
    * `name` - The name of the new category.")]
    async fn add_category(
        &self,
        Parameters(payload): Parameters<AddCategoryCommand>,
    ) -> Result<CallToolResult, McpError> {
        use crate::cmd::category::AddCategoryRequest;
        let request = AddCategoryRequest {
            name: payload.name.clone(),
            parent_id: payload.parent_id.clone(),
        };
        let category_id = category::add_category(request);
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&category_id).unwrap(),
        )]))
    }

    #[tool(description = "Get uncategorized apps.
    Returns a list of apps that have not been assigned to any category, each with app ID and path.
    # Arguments

    * `limit` - Optional limit to return only N apps. If not provided, defaults to 20.")]
    async fn get_uncategorized_apps(
        &self,
        Parameters(payload): Parameters<GetUncategorizedAppsCommand>,
    ) -> Result<CallToolResult, McpError> {
        use crate::cmd::category::GetUncategorizedAppsRequest;
        let limit = payload.limit.unwrap_or(20);
        let request = GetUncategorizedAppsRequest {
            offset: 0,
            limit,
            keyword: None,
        };
        let uncategorized = category::get_uncategorized_apps(request).await;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&uncategorized).unwrap(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for McpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build()
        )
        .with_protocol_version(ProtocolVersion::V_2024_11_05)
        .with_server_info(Implementation::from_build_env())
        .with_instructions("This server provides a counter tool that can increment and decrement values. The counter starts at 0 and can be modified using the 'increment' and 'decrement' tools. Use 'get_value' to check the current count.")
    }

    async fn initialize(
        &self,
        _request: rmcp::model::InitializeRequestParams,
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
