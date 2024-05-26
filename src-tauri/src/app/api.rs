/// All time is transmitted in milliseconds
use std::collections::HashMap;
use std::path::Path;

use super::data::FileDetail;
use super::data::Tick;
use super::file_version::file_version;
use crate::app::data::FocusRecord;
use crate::app::persist::get_path_by_id;
use crate::app::persist::read_records_by_datetime;
use chrono::DateTime;
use chrono::Duration;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommandError {
    message: String,
}

impl CommandError {
    fn new(message: String) -> CommandError {
        CommandError { message }
    }
}

#[tauri::command]
pub fn duration_aggregate(start_millis: u64, end_millis: u64) -> HashMap<u64, u64> {
    let records = read_records_by_datetime(start_millis, end_millis);
    duration_by_id(records.into_iter().flatten().collect())
}

/// `time_zone_offset` base on the time zone of user, and it is the offset milliseconds of UTC time.
/// The return value is a map of day and duration. The key is the day from `UNIX_EPOCH` which base on the user's time zone.
/// Remember receive with timezone.
#[tauri::command]
pub fn duration_by_day(
    start_millis: u64,
    end_millis: u64,
    time_zone_offset: i64,
) -> HashMap<usize, u64> {
    let one_day_millis = Duration::days(1).num_milliseconds();
    // Hard to think, for utc time, > -offset is the same day of user's timezone.
    let (threshold, compensation) = if time_zone_offset > 0 {
        (one_day_millis - time_zone_offset, 1)
    } else {
        (-time_zone_offset, 0)
    };
    let threshold = Tick::from_millis(threshold as u64);
    let records = read_records_by_datetime(start_millis, end_millis);
    let start_day = (start_millis as i64) / one_day_millis;
    let mut ret = HashMap::new();
    for (idx, records_by_day) in records.iter().enumerate() {
        if records_by_day.is_empty() {
            continue;
        };
        let day = start_day as usize + idx + compensation;
        let part = records_by_day
            .binary_search_by_key(&threshold, |x| x.focus_at())
            .unwrap_or_else(|x| x);
        let mut pre_day_total: Tick = records_by_day[..part.saturating_sub(1)]
            .iter()
            .map(|x| x.duration())
            .sum();
        let mut cur_day_total: Tick = records_by_day[part..].iter().map(|x| x.duration()).sum();
        // The previous position record of partition point may include both yesterday and today.
        if part - 1 < records_by_day.len() {
            let record = &records_by_day[part - 1];
            if record.blur_at() < threshold {
                pre_day_total += record.duration();
            } else {
                pre_day_total += threshold - record.focus_at();
                cur_day_total += record.blur_at() - threshold;
            }
        }
        *ret.entry(day - 1).or_insert(0) += pre_day_total.to_millis();
        *ret.entry(day).or_insert(0) += cur_day_total.to_millis();
    }
    ret
}

#[tauri::command]
pub fn file_detail(id: usize) -> FileDetail {
    let path = get_path_by_id(id);
    let version = file_version(&path);
    let name = version
        .as_ref()
        .map(|x| x.product_name.to_owned())
        .flatten()
        .or(Path::new(&path)
            .file_name()
            .map(|x| x.to_str().map(|x| x.to_owned()))
            .flatten())
        .unwrap_or(path.to_owned());
    FileDetail {
        name,
        id,
        version,
        path,
    }
}

/// Calculate duration based on app_id.
/// The map key is app id, value is duration in milliseconds.
pub fn duration_by_id(records: Vec<FocusRecord>) -> HashMap<u64, u64> {
    let mut map = HashMap::new();

    for record in records.iter() {
        *map.entry(record.app_id()).or_insert(0) += record.duration().to_millis();
    }
    return map;
}
