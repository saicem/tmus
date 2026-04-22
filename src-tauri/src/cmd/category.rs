use std::sync::{Arc, Mutex};

use crate::{
    cmd::app_detail::{get_app_detail_cache, FileDetail},
    state::category::{self, CategoryId, CategoryNode, CategorySimple, ROOT_NODE_ID},
};
use serde::{Deserialize, Serialize};
use tauri::command;
use tmus_engine::models::AppId;

use crate::state::category::UncategorizedAppsResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCategoryRequest {
    pub name: String,
    pub parent_id: CategoryId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryRequest {
    pub id: CategoryId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCategoryRequest {
    pub id: CategoryId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAppCategoryRequest {
    pub app_id: usize,
    pub category_id: CategoryId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAppCategoryRequest {
    pub app_id: AppId,
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
pub fn get_category_tree() -> Arc<Mutex<CategoryNode>> {
    category::get_category_tree()
}

#[command]
#[tracing::instrument]
pub fn get_all_categories() -> Vec<CategorySimple> {
    category::get_category_detail_map()
        .values()
        .map(|x| (&*x.lock().unwrap()).into())
        .filter(|x: &CategorySimple| x.id != ROOT_NODE_ID)
        .collect::<Vec<_>>()
}

#[command(async)]
#[tracing::instrument]
pub async fn get_category_apps(category_id: CategoryId) -> Result<Vec<FileDetail>, String> {
    let app_map = get_app_detail_cache().await;
    let app_details = category::get_category_detail_map()
        .get(&category_id)
        .unwrap()
        .lock()
        .unwrap()
        .app_ids
        .iter()
        .filter_map(|app_id| app_map.get(app_id).map(ToOwned::to_owned))
        .collect::<Vec<_>>();
    Ok(app_details)
}

#[command]
#[tracing::instrument]
pub fn add_category(request: AddCategoryRequest) -> Result<(), String> {
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
    category::set_app_category(request.app_id, request.category_id)
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
