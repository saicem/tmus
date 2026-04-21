use serde::{Deserialize, Serialize};

use crate::state::scheme::{
    add_statistic_scheme_item, delete_statistic_scheme_item, get_statistic_scheme,
    save_statistic_scheme, StatisticScheme, StatisticSchemeDetail,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddSchemeItemRequest {
    pub name: String,
    pub detail: StatisticSchemeDetail,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddSchemeItemResponse {
    pub id: u64,
}

#[tauri::command]
pub fn get_statistic_scheme_list() -> StatisticScheme {
    get_statistic_scheme().clone()
}

#[tauri::command]
pub fn add_statistic_scheme(request: AddSchemeItemRequest) -> AddSchemeItemResponse {
    let id = get_statistic_scheme().items.len() as u64;
    add_statistic_scheme_item(request.name, request.detail);
    AddSchemeItemResponse { id }
}

#[tauri::command]
pub fn delete_statistic_scheme(id: u64) -> Result<(), String> {
    delete_statistic_scheme_item(&id)
}

#[tauri::command]
pub fn save_statistic_scheme_manual() {
    save_statistic_scheme();
}
