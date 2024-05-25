use std::collections::HashMap;
use std::path::Path;

use super::data::FileDetail;
use super::file_version::file_version;
use crate::app::data::FocusRecord;
use crate::app::persist::get_path_by_id;
use crate::app::persist::read_records_by_datetime;
use chrono::DateTime;
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
    let ret = duration_by_id(records.into_iter().flatten().collect());
    Ok(ret)
}

#[tauri::command]
pub fn file_version_by_id(id: usize) -> FileDetail {
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
