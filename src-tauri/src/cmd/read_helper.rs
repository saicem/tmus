use std::cmp::{max, min};
use tmus_engine::models::FocusRecord;
use tmus_engine::storage;
use tmus_engine::util::Timestamp;
use tracing::debug;

// For compute convenience, remember convert them back after compute.
pub fn timezone_convert(mut vec: Vec<FocusRecord>, timezone_offset: Timestamp) -> Vec<FocusRecord> {
    for item in vec.iter_mut() {
        item.focus_at = item.focus_at - timezone_offset;
        item.blur_at = item.blur_at - timezone_offset;
    }
    vec
}

/// Reads all focus records that fall within the specified time range.
/// If a record not entirely falls inside the time range, it will be trimmed to fit.
///
/// # Arguments
/// - `start_timestamp`: The start of the time range, inclusive.
/// - `end_timestamp`: The end of the time range, inclusive.
///
/// # Returns
/// A vector of `FocusRecord` instances.
pub fn read_by_timestamp(start_timestamp: Timestamp, end_timestamp: Timestamp) -> Vec<FocusRecord> {
    debug_assert!(
        start_timestamp <= end_timestamp,
        "Start_timestamp must be less than or equal to end_timestamp"
    );
    let rough_records = storage::read_by_timestamp(start_timestamp, end_timestamp);
    debug!(
        "Read rough records, start: {:?}, end: {:?}, len: {:?}",
        start_timestamp,
        end_timestamp,
        rough_records.len()
    );
    trim_focus_records(rough_records, start_timestamp, end_timestamp)
}

/// Trims focus records to retain only those within the specified time range.
///
/// This function accepts a vector of focus records along with start and end times in milliseconds.
/// It returns a new vector containing only the records that fall within the given time span.
/// If the original record vector is empty or the specified time range lies entirely outside existing records,
/// an empty vector is returned.
///
/// Arguments：
/// - `records`: The original vector of focus records.
/// - `start_timestamp`: The specified start time in milliseconds.
/// - `end_timestamp`: The specified end time in milliseconds.
///
/// Returns:
/// - A new vector containing focus records within the defined time range.
fn trim_focus_records(
    mut records: Vec<FocusRecord>,
    start_timestamp: Timestamp,
    end_timestamp: Timestamp,
) -> Vec<FocusRecord> {
    if records.is_empty() {
        return records;
    }
    // If the record blur time equals start range, after trimming, the record duration is zero.
    let start_index = records
        .binary_search_by_key(&start_timestamp, |x| x.blur_at)
        .map(|x| x + 1)
        .unwrap_or_else(|x| x);
    let end_index = records
        .binary_search_by_key(&end_timestamp, |x| x.focus_at)
        .unwrap_or_else(|x| x);
    if start_index >= end_index || start_index >= records.len() {
        return vec![];
    }
    records = records[start_index..end_index].to_vec();
    records
        .first_mut()
        .map(|x| x.focus_at = max(x.focus_at, start_timestamp));
    records
        .last_mut()
        .map(|x| x.blur_at = min(x.blur_at, end_timestamp));
    records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_focus_records() {
        let records = vec![
            ms(500, 1000),
            ms(1500, 2000),
            ms(2500, 2600),
            ms(2600, 2700),
            ms(2900, 3000),
        ];

        // All include.
        sub_test_trim_focus_records(0, 3000, records.clone(), records.clone());
        // Wrong input.
        sub_test_trim_focus_records(2000, 1000, records.clone(), vec![]);
        // Boundary test.
        sub_test_trim_focus_records(1000, 1500, records.clone(), vec![]);
        sub_test_trim_focus_records(500, 1500, records.clone(), vec![ms(500, 1000)]);
        sub_test_trim_focus_records(1000, 2000, records.clone(), vec![ms(1500, 2000)]);
        sub_test_trim_focus_records(1000, 2500, records.clone(), vec![ms(1500, 2000)]);
        sub_test_trim_focus_records(1000, 2500, records.clone(), vec![ms(1500, 2000)]);
        // Trim test.
        sub_test_trim_focus_records(777, 888, records.clone(), vec![ms(777, 888)]);
        sub_test_trim_focus_records(
            800,
            2620,
            records,
            vec![
                ms(800, 1000),
                ms(1500, 2000),
                ms(2500, 2600),
                ms(2600, 2620),
            ],
        );
    }

    fn sub_test_trim_focus_records(
        start: i64,
        end: i64,
        records: Vec<FocusRecord>,
        result: Vec<FocusRecord>,
    ) {
        let trimmed_records = trim_focus_records(records, start, end);
        assert_eq!(trimmed_records, result);
    }

    fn ms(focus_at: i64, blur_at: i64) -> FocusRecord {
        FocusRecord {
            id: 0,
            focus_at,
            blur_at,
        }
    }
}
