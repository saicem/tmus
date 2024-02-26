use crate::app::file::FocusRecord;
use std::collections::HashMap;
use tauri;

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
