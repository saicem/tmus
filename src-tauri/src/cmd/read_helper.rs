use crate::engine::{data::Millisecond, Engine, FocusRecord};
use std::cmp::{max, min};

/// Reads all focus records that fall within the specified time range.
/// If a record not entirely falls inside the time range, it will be trimmed to fit.
///
/// # Arguments
/// - `start_millis`: The start of the time range, inclusive.
/// - `end_millis`: The end of the time range, inclusive.
///
/// # Returns
/// A vector of `FocusRecord` instances.
pub fn read_by_timestamp(start_millis: Millisecond, end_millis: Millisecond) -> Vec<FocusRecord> {
    if start_millis >= end_millis {
        return vec![];
    }
    let rough_records = Engine::read_by_time(start_millis, end_millis);
    trim_focus_records(rough_records, start_millis, end_millis)
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
/// - `start_millis`: The specified start time in milliseconds.
/// - `end_millis`: The specified end time in milliseconds.
///
/// Returns:
/// - A new vector containing focus records within the defined time range.
fn trim_focus_records(
    mut records: Vec<FocusRecord>,
    start_millis: Millisecond,
    end_millis: Millisecond,
) -> Vec<FocusRecord> {
    if records.is_empty() {
        return records;
    }
    // If the record blur time equals start range, after trimming, the record duration is zero.
    let start_index = records
        .binary_search_by_key(&start_millis, |x| x.blur_at)
        .map(|x| x + 1)
        .unwrap_or_else(|x| x);
    let end_index = records
        .binary_search_by_key(&end_millis, |x| x.focus_at)
        .unwrap_or_else(|x| x);
    if start_index >= end_index || start_index >= records.len() {
        return vec![];
    }
    records = records[start_index..end_index].to_vec();
    records
        .first_mut()
        .map(|x| x.focus_at = max(x.focus_at, start_millis));
    records
        .last_mut()
        .map(|x| x.blur_at = min(x.blur_at, end_millis));
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
        let start_millis = Millisecond::from_millis(start);
        let end_millis = Millisecond::from_millis(end);
        let trimmed_records = trim_focus_records(records, start_millis, end_millis);
        assert_eq!(trimmed_records, result);
    }

    fn ms(focus_at: i64, blur_at: i64) -> FocusRecord {
        FocusRecord::new(
            0,
            Millisecond::from_millis(focus_at),
            Millisecond::from_millis(blur_at),
        )
    }
}
