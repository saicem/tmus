use crate::config::Config;
use crate::engine::data::Millisecond;
use crate::engine::{Engine, FocusRecord};
use crate::util::file_version;
use file_detail::FileDetail;
use log::info;
use read::read_by_timestamp;
use std::collections::HashMap;
use std::path::Path;

mod file_detail;
mod read;

#[tauri::command]
pub fn raw_record(
    start_millis: Millisecond,
    end_millis: Millisecond,
) -> Result<Vec<FocusRecord>, String> {
    Ok(read_by_timestamp(start_millis, end_millis))
}

#[tauri::command]
pub fn read_reverse(
    cursor: Option<u64>,
    count: u64,
) -> Result<(Vec<FocusRecord>, Option<u64>), String> {
    Ok(read::read_reverse(cursor, count))
}

#[tauri::command]
pub fn duration_aggregate(
    start_millis: Millisecond,
    end_millis: Millisecond,
) -> Result<HashMap<usize, Millisecond>, String> {
    let records = read_by_timestamp(start_millis, end_millis);
    Ok(duration_by_id(records.into_iter().collect()))
}

/// Calculates the duration per day given a time range and timezone offset.
///
/// # Parameters
/// - `start`: The start timestamp in milliseconds.
/// - `end`: The end timestamp in milliseconds.
/// - `time_zone_offset`: The timezone offset in milliseconds.
///
/// # Returns
/// A `Result` containing either:
/// - A `HashMap` where keys are days (in i64 format) and values are durations in milliseconds for that day.
/// - A `String` containing an error message if an error occurs.
#[tauri::command]
pub fn duration_by_day(
    start_millis: Millisecond,
    end_millis: Millisecond,
    time_zone_offset: Millisecond,
) -> Result<HashMap<i64, Millisecond>, String> {
    let records = read_by_timestamp(start_millis, end_millis);
    info!(
        "records len: {:?}, start: {:?}, end: {:?}",
        records.len(),
        start_millis,
        end_millis
    );
    let mut ret = HashMap::new();
    for mut record in records.into_iter() {
        record.focus_at += time_zone_offset;
        record.blur_at += time_zone_offset;
        let start_day = record.focus_at.as_days();
        let end_day = record.focus_at.as_days();
        if start_day == end_day {
            *ret.entry(start_day).or_insert(Millisecond::ZERO) += record.duration();
        } else {
            *ret.entry(start_day).or_insert(Millisecond::ZERO) +=
                Millisecond::ONE_DAY - record.focus_at.get_day_offset();
            *ret.entry(end_day).or_insert(Millisecond::ZERO) += record.blur_at.get_day_offset();
        }
    }
    Ok(ret)
}

#[tauri::command]
pub fn file_detail(id: usize) -> Result<FileDetail, String> {
    let path = Engine::get_path_by_id(id).unwrap();
    let version = file_version::query_file_version(&path);
    let name = version
        .as_ref()
        .map(|x| x.product_name.to_owned())
        .flatten()
        .or(Path::new(&path)
            .file_name()
            .map(|x| x.to_str().map(|x| x.to_owned()))
            .flatten())
        .unwrap_or(path.to_owned());
    Ok(FileDetail {
        name,
        id,
        version,
        path,
    })
}

#[tauri::command]
pub fn get_app_config() -> Config {
    Config::get()
}

/// Calculate duration based on app_id.
pub fn duration_by_id(records: Vec<FocusRecord>) -> HashMap<usize, Millisecond> {
    let mut map = HashMap::new();
    for record in records.iter() {
        *map.entry(record.id).or_insert(Millisecond::ZERO) += record.duration();
    }
    map
}
