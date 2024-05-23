use chrono::{DateTime, Utc};

use crate::app::data::{FocusRecord, TmusTick};
use crate::app::persist::{file_index, file_record};

/// Query records between start day and end day.
/// `start_day` must be less than `end_day`.
pub fn read_records_by_day(start_day: u64, end_day: u64) -> Vec<FocusRecord> {
    let (start_index, end_index) = file_index::query_index(start_day, end_day + 1);
    if start_index == u64::MAX {
        return Vec::new();
    }
    if end_index == u64::MAX {
        return file_record::read_to_end(start_day);
    }
    file_record::read_record(start_index, end_index)
}

pub fn read_records_by_datetime(
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> Vec<FocusRecord> {
    let start = TmusTick::from_date_time(start_datetime);
    let end = TmusTick::from_date_time(end_datetime);
    let records = read_records_by_day(start.day(), end.day());
    if records.is_empty() {
        return records;
    }
    let start_index = records
        .binary_search_by_key(&start.tick, |x| x.focus_at())
        .unwrap_or_else(|pos| pos);
    let end_index = records
        .binary_search_by_key(&end.tick, |x| x.blur_at())
        .unwrap_or_else(|pos| pos.saturating_sub(1));
    println!("{},{}", start_index, end_index);
    records[start_index..=end_index].to_vec()
}
