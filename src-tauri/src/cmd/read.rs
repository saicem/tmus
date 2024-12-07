use crate::engine::data::{CursorPosition, ReadDirection};
use crate::engine::{data::Millisecond, Engine, FocusRecord};
use std::cmp::{max, min};

#[tauri::command]
pub fn read_by_timestamp(start_millis: Millisecond, end_millis: Millisecond) -> Vec<FocusRecord> {
    if start_millis >= end_millis {
        return vec![];
    }
    let rough_records = Engine::read_by_time(start_millis, end_millis);
    trim_focus_records(rough_records, start_millis, end_millis)
}

/// This function is designed to retrieve a set of records in reverse order based on a given cursor position and the number of records to retrieve.
/// It is particularly useful in scenarios where data needs to be displayed or processed in reverse order, such as displaying the latest messages at the top of a chat application.
///
/// Parameters:
/// - `cursor`: An optional parameter representing the starting point of the query. It is usually the ID or position of the last record retrieved in the previous query.
///   If this is the first query, the cursor can be `None`.
/// - `count`: A mandatory parameter indicating the number of records to retrieve. It is used to control the amount of data retrieved in a single query, thereby optimizing performance and controlling memory usage.
///
/// Returns:
/// - A tuple containing two elements:
///   1. A vector of `FocusRecord` instances, representing the set of records retrieved in reverse order.
///   2. An `u64` value representing the cursor position for the next query. If there are no more records to retrieve, this value will be `None`.
pub fn read_reverse(cursor: Option<u64>, count: u64) -> (Vec<FocusRecord>, Option<u64>) {
    let (records, new_cursor) = Engine::read_by_cursor(
        match cursor {
            None => CursorPosition::End,
            Some(idx) => CursorPosition::Middle(idx),
        },
        count,
        ReadDirection::Backward,
    );
    let ret_cursor = match new_cursor {
        CursorPosition::Start => None,
        CursorPosition::End => None,
        CursorPosition::Middle(idx) => Some(idx),
    };
    (records, ret_cursor)
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
