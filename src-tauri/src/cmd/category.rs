use crate::state::category;
use serde::{Deserialize, Serialize};
use tauri::command;

use crate::state::category::UncategorizedAppsResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCategoryRequest {
    pub name: String,
    pub parent_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryRequest {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCategoryRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAppCategoryRequest {
    pub app_id: usize,
    pub category_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAppCategoryRequest {
    pub app_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUncategorizedAppsRequest {
    pub offset: usize,
    pub limit: usize,
    pub keyword: Option<String>,
}

#[command]
#[tracing::instrument]
pub fn get_categories() -> category::CategoryNode {
    category::get_category_all()
}

#[command]
#[tracing::instrument]
pub fn add_category(request: AddCategoryRequest) -> Result<category::CategorySimple, String> {
    category::add_category(request.parent_id, request.name)
}

#[command]
#[tracing::instrument]
pub fn update_category(request: UpdateCategoryRequest) -> Result<(), String> {
    category::rename_category(request.id, request.name)
}

#[command]
#[tracing::instrument]
pub fn delete_category(request: DeleteCategoryRequest) -> Result<(), String> {
    category::delete_category(request.id)
}

#[command]
#[tracing::instrument]
pub fn set_app_category(request: SetAppCategoryRequest) -> Result<(), String> {
    category::set_app_category(request.app_id, &request.category_id)
}

#[command]
#[tracing::instrument]
pub fn remove_app_from_category(request: RemoveAppCategoryRequest) -> Result<(), String> {
    category::remove_app_from_category(request.app_id);
    Ok(())
}

#[command(async)]
#[tracing::instrument]
pub async fn get_uncategorized_apps(
    request: GetUncategorizedAppsRequest,
) -> Result<UncategorizedAppsResult, String> {
    category::get_uncategorized_apps(request.offset, request.limit, request.keyword).await
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryAppsRequest {
    pub category_id: String,
}
