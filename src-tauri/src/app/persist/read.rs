use chrono::{DateTime, Utc};

use crate::app::data::focus_record::FocusRecord;
use crate::app::data::tmus_tick::TmusTick;

#[tauri::command]
pub fn read_record_by_index(start: u64, end: u64) -> Vec<FocusRecord> {
    crate::app::persist::core::FILE_RECORD
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .read_record(start, end)
}

/// Query records between start day and end day.
/// `start_day` must be less than `end_day`.
#[tauri::command]
pub fn read_records_by_day(start_day: u64, end_day: u64) -> Vec<FocusRecord> {
    let (mut start_index, mut end_index) = crate::app::persist::core::FILE_INDEX
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .query_index(start_day, end_day);
    if start_index == u64::MAX {
        return Vec::new();
    }

    let mut file_app = crate::app::persist::core::FILE_RECORD
        .get()
        .unwrap()
        .lock()
        .unwrap();
    if end_index == u64::MAX {
        return file_app.read_to_end(start_index);
    }
    file_app.read_record(start_index, end_index)
}

#[tauri::command]
pub fn read_records_by_datetime(
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> Vec<FocusRecord> {
    let start = TmusTick::from_date_time(start_datetime);
    let end = TmusTick::from_date_time(end_datetime);
    let records = read_records_by_day(start.day(), end.day());
    let start_index = records
        .binary_search_by_key(&start.tick, |x| x.focus_at())
        .unwrap_or_else(|pos| pos);
    let end_index = records
        .binary_search_by_key(&end.tick, |x| x.blur_at())
        .unwrap_or_else(|pos| pos);
    records[start_index..=end_index].to_vec()
}
