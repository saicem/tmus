use std::cmp::{max, min};

use crate::engine::{FocusRecord, Millisecond, ENGINE};

/// Query records between start day and end day, both day are include.
// fn read_by_day(start_day: Day, end_day: Day) -> Vec<FocusRecord> {
//     ENGINE.get().unwrap().read_records(start, end);
//     let index_vec = file_index::query_index(start_day, end_day + 1);
//     let start_index = index_vec[0];
//     let end_index = index_vec.last().unwrap().to_owned();
//     if start_index == u64::MAX {
//         return Vec::new();
//     }
//     let records = file_record::read_record(start_index, end_index);
//     let records_len = records.len();
//     let group_index: Vec<usize> = index_vec
//         .iter()
//         .map(|x| min(((x - start_index) / 8) as usize, records_len))
//         .collect();
//     let mut ret = Vec::new();
//     for idx in 0..group_index.len() - 1 {
//         let start = group_index[idx];
//         let end = group_index[idx + 1];
//         ret.push(records[start..end].to_owned());
//     }
//     ret
// }

fn read_by_day(start_day: i64, end_day: i64) -> Vec<FocusRecord> {
    ENGINE.get().unwrap().read_rough_records(start_day, end_day)
}

pub fn read_by_timestamp(start_millis: Millisecond, end_millis: Millisecond) -> Vec<FocusRecord> {
    let start_day = start_millis.as_days();
    let end_day = end_millis.as_days();
    let mut records = read_by_day(start_day, end_day);
    if records.is_empty() {
        return records;
    }
    let start_index = records
        .binary_search_by_key(&start_millis, |x| x.blur_at)
        .unwrap_or_else(|x| x);

    let end_index = records
        .binary_search_by_key(&end_millis, |x| x.focus_at)
        .unwrap_or_else(|x| x);
    records = records[start_index..end_index].to_vec();
    records
        .first_mut()
        .map(|x| x.focus_at = max(x.focus_at, start_millis));
    records
        .last_mut()
        .map(|x| x.blur_at = min(x.blur_at, end_millis));
    records
}

pub fn read_reverse(cursor: Option<u64>, count: u64) {
    todo!()
}
