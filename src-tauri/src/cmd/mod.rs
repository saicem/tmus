use crate::cmd::read_helper::read_by_timestamp;
use crate::util;
use serde::{Deserialize, Serialize};
use tmus_engine::models::EngineMeta;
use tmus_engine::tracking::focus_index;
use tmus_engine::util::Timestamp;
use tmus_engine::{tracking, FocusRecord};

pub mod app_detail;
pub mod app_duration_area;
pub mod duration;
mod read_helper;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TmusMeta {
    pub initial_timestamp: Timestamp,
    pub engine_version: String,
    pub tmus_version: String,
}

#[tauri::command]
pub fn get_raw_record(start_timestamp: Timestamp, end_timestamp: Timestamp) -> Vec<FocusRecord> {
    read_by_timestamp(start_timestamp, end_timestamp)
}

#[tauri::command]
pub async fn show_in_folder(path: String) {
    util::show_in_folder(path)
}

#[tauri::command]
pub async fn get_tmus_meta() -> TmusMeta {
    let EngineMeta {
        initial_timestamp,
        engine_version,
    } = tracking::get_tmus_meta();
    TmusMeta {
        initial_timestamp,
        engine_version,
        tmus_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[tauri::command]
pub async fn focus_index_record() -> Vec<focus_index::FileIndexRecord> {
    focus_index::all_record()
}
