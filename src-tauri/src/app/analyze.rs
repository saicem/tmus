use std::collections::HashMap;

use chrono::DateTime;
use serde::Serialize;

use crate::app::data::FocusRecord;
use crate::app::persist::read_records_by_datetime;

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
pub fn duration_aggregate(
    start_str: String,
    end_str: String,
) -> Result<HashMap<u64, u64>, CommandError> {
    let parse_from_str = |x: String| {
        DateTime::parse_from_rfc3339(&x)
            .map(|x| x.to_utc())
            .map_err(|err| CommandError::new(err.to_string()))
    };
    let start = parse_from_str(start_str)?;
    let end = parse_from_str(end_str)?;
    let records = read_records_by_datetime(start, end);
    Ok(duration_by_id(records))
}

/// Calculate duration based on app_id.
///
/// # Examples
///
///  ```
///  let mut records = Vec::new();
///  records.push(FocusRecord::new(1, 0, 2));
///  records.push(FocusRecord::new(1, 0, 3));
///  records.push(FocusRecord::new(2, 0, 1));
///  let map = duration_by_id(records);
///
///  assert_eq!(map.get(&1), 5);
///  assert_eq!(map.get(&2), 1);
/// ```
pub fn duration_by_id(records: Vec<FocusRecord>) -> HashMap<u64, u64> {
    records.iter().map(|x| (x.app_id(), x.duration())).collect()
}
