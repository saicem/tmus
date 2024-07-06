use crate::engine::r#type::Millisecond;
use crate::engine::{FocusRecord, ENGINE};
use file_detail::FileDetail;
use read::read_by_timestamp;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::path::Path;

mod file_detail;
mod file_version;
mod read;

const ONE_DAY_SECS: u64 = 24 * 60 * 60;
const ONE_DAY_MILLIS: i64 = 24 * 60 * 60 * 1000;

#[tauri::command]
pub fn raw_record(start: Millisecond, end: Millisecond) -> Result<Vec<FocusRecord>, String> {
    Ok(read_by_timestamp(start, end))
}

#[tauri::command]
pub fn duration_aggregate(
    start_millis: Millisecond,
    end_millis: Millisecond,
) -> Result<HashMap<usize, i64>, String> {
    let records = read_by_timestamp(start_millis, end_millis);
    Ok(duration_by_id(records.into_iter().collect()))
}

/// The return value is a map of day and duration. The key is the day from `UNIX_EPOCH` which base on the user's time zone.
#[tauri::command]
pub fn duration_by_day(
    start: Millisecond,
    end: Millisecond,
    time_zone_offset: Millisecond,
) -> Result<HashMap<i64, i64>, String> {
    let records = read_by_timestamp(start, end);
    let mut cur_pat_millis = start + time_zone_offset;
    let mut cur_day = (start + time_zone_offset) / ONE_DAY_MILLIS;
    let mut ret = HashMap::new();
    for record in records.iter() {
        if record.blur_at >= cur_pat_millis {
            *ret.entry(cur_day).or_insert(0) += min(0, cur_pat_millis - record.focus_at);
            *ret.entry(cur_day + 1).or_insert(0) +=
                record.blur_at - max(record.focus_at, cur_pat_millis);
            cur_day += 1;
            cur_pat_millis += ONE_DAY_MILLIS;
        } else {
            *ret.entry(cur_day).or_insert(0) += record.duration()
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
pub fn duration_by_id(records: Vec<FocusRecord>) -> HashMap<usize, i64> {
    let mut map = HashMap::new();

    for record in records.iter() {
        *map.entry(record.id).or_insert(0) += record.duration();
    }
    return map;
}