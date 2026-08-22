use crate::app::update;

use crate::util;
use serde::{Deserialize, Serialize};
use tauri::ipc::Invoke;
use tauri::Wry;
use tmus_engine::models::EngineMeta;
use tmus_engine::storage;
use tmus_engine::util::Timestamp;

pub mod app_detail;
pub mod app_duration_area;
pub mod category;
pub mod duration;
pub mod read_helper;
pub mod statistic;

pub use read_helper::read_by_timestamp;

pub fn handler() -> impl Fn(Invoke<Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        show_in_folder,
        get_tmus_meta,
        crate::state::get_app_config,
        crate::state::set_app_config,
        crate::state::get_app_rule,
        crate::state::set_app_rule,
        app_detail::get_app_detail,
        app_detail::get_all_app_detail,
        update::fetch_update,
        update::install_update,
        app_duration_area::get_app_duration_area,
        duration::get_duration_by_id,
        duration::query_duration_statistic,
        category::get_category_tree,
        category::get_all_categories,
        category::add_category,
        category::update_category,
        category::delete_category,
        category::set_app_category,
        category::remove_app_from_category,
        category::get_uncategorized_apps,
        category::get_category_apps,
        statistic::get_app_total_duration,
        statistic::get_app_usage_days,
        statistic::get_category_total_duration,
        statistic::get_category_usage_days,
        statistic::get_category_usage_rhythm,
    ]
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TmusMeta {
    pub initial_timestamp: Timestamp,
    pub engine_version: String,
    pub tmus_version: String,
}

#[tauri::command]
#[tracing::instrument]
pub fn show_in_folder(path: String) {
    util::show_in_folder(&path)
}

#[tauri::command]
#[tracing::instrument]
pub fn get_tmus_meta() -> TmusMeta {
    let EngineMeta {
        initial_timestamp,
        engine_version,
    } = storage::get_tmus_meta();
    TmusMeta {
        initial_timestamp,
        engine_version,
        tmus_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}
