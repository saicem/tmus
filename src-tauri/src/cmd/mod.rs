use crate::cmd::read_helper::read_by_timestamp;
use tmus_engine::models::AppMeta;
use tmus_engine::tracking::focus_index;
use tmus_engine::util::Timestamp;
use tmus_engine::{tracking, FocusRecord};
use crate::util;

pub mod app_detail;
pub mod app_duration_area;
pub mod duration;
pub mod duration_statistic;
mod read_helper;

#[tauri::command]
pub fn get_raw_record(start_timestamp: Timestamp, end_timestamp: Timestamp) -> Vec<FocusRecord> {
    read_by_timestamp(start_timestamp, end_timestamp)
}

#[tauri::command]
pub async fn show_in_folder(path: String) {
    util::show_in_folder(path)
}

#[tauri::command]
pub async fn get_tmus_meta() -> AppMeta {
    tracking::get_tmus_meta()
}

#[tauri::command]
pub async fn focus_index_record() -> Vec<focus_index::FileIndexRecord> {
    focus_index::all_record()
}
