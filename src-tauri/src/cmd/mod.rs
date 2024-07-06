use crate::engine::Millisecond;
use crate::engine::{FocusRecord, ENGINE};
use file_detail::FileDetail;
use read::read_by_timestamp;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::path::Path;

mod file_detail;
mod file_version;
mod read;

#[tauri::command]
pub fn raw_record(start: Millisecond, end: Millisecond) -> Result<Vec<FocusRecord>, String> {
    Ok(read_by_timestamp(start, end))
}

#[tauri::command]
pub fn read_reverse(cursor: Option<u64>, count: u64) -> Result<(Vec<FocusRecord>, u64), String> {
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
    start: Millisecond,
    end: Millisecond,
    time_zone_offset: Millisecond,
) -> Result<HashMap<i64, Millisecond>, String> {
    let records = read_by_timestamp(start, end);
    let mut cur_pat_millis = start + time_zone_offset;
    let mut cur_day = (start + time_zone_offset).as_days();
    let mut ret = HashMap::new();
    for record in records.iter() {
        if record.blur_at >= cur_pat_millis {
            *ret.entry(cur_day).or_insert(Millisecond::ZERO) +=
                min(Millisecond::ZERO, cur_pat_millis - record.focus_at);
            *ret.entry(cur_day + 1).or_insert(Millisecond::ZERO) +=
                record.blur_at - max(record.focus_at, cur_pat_millis);
            cur_day += 1;
            cur_pat_millis += Millisecond::from_days(1);
        } else {
            *ret.entry(cur_day).or_insert(Millisecond::ZERO) += record.duration()
        }
    }
    Ok(ret)
}

#[tauri::command]
pub fn file_detail(id: usize) -> Result<FileDetail, String> {
    let path = ENGINE.get().unwrap().get_path_by_id(id);
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

/// Calculate duration based on app_id.
pub fn duration_by_id(records: Vec<FocusRecord>) -> HashMap<usize, Millisecond> {
    let mut map = HashMap::new();

    for record in records.iter() {
        *map.entry(record.id).or_insert(Millisecond::ZERO) += record.duration();
    }
    return map;
}
