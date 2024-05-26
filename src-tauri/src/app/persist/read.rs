use std::cmp::min;

use crate::app::data::{FocusRecord, Tick};
use crate::app::persist::{file_index, file_record};

/// Query records between start day and end day, both day are include.
/// `start_day` must be less than `end_day`.
/// Return value are group by the day.
fn read_records_by_day(start_day: u64, end_day: u64) -> Vec<Vec<FocusRecord>> {
    // If you query 2 days, you need 3 index, so end_day + 1.
    let index_vec = file_index::query_index(start_day, end_day + 1);
    let start_index = index_vec[0];
    let end_index = index_vec.last().unwrap().to_owned();
    if start_index == u64::MAX {
        return Vec::new();
    }
    let records = file_record::read_record(start_index, end_index);
    let records_len = records.len();
    let group_index: Vec<usize> = index_vec
        .iter()
        .map(|x| min(((x - start_index) / 8) as usize, records_len))
        .collect();
    let mut ret = Vec::new();
    for idx in 0..group_index.len() - 1 {
        let start = group_index[idx];
        let end = group_index[idx + 1];
        ret.push(records[start..end].to_owned());
    }
    ret
}

pub fn read_records_by_datetime(start_millis: u64, end_millis: u64) -> Vec<Vec<FocusRecord>> {
    let start = Tick::from_millis(start_millis);
    let end = Tick::from_millis(end_millis);
    let mut records = read_records_by_day(start.day(), end.day());
    if records.is_empty() {
        return records;
    }
    // Extract records within the time range.
    let first = &records[0];
    let index = first
        .binary_search_by_key(&start.day_tick(), |x| x.blur_at())
        .unwrap_or_else(|x| x);
    records[0] = records[0][index..].to_vec();

    let last_index = records.len() - 1;
    let last = &records[last_index];
    let index = last
        .binary_search_by_key(&end.day_tick(), |x| x.focus_at())
        .unwrap_or_else(|x| x);
    records[last_index] = records[last_index][..index].to_vec();
    records
}
