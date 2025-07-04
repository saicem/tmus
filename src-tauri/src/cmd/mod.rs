use crate::cmd::data::FileDetail;
use crate::cmd::duration_calculate_helper::{group_by_day, group_by_day_id, group_by_id};
use crate::engine::alpha::focus_index;
use crate::engine::data::{AppId, AppMeta, Millisecond};
use crate::engine::{alpha, FocusRecord};
use crate::util;
use base64::engine::general_purpose;
use base64::Engine as _;
use image::ImageFormat;
use log::info;
use read_helper::read_by_timestamp;
use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;
use std::path::Path;

mod data;
mod duration_calculate_helper;
mod read_helper;

#[tauri::command]
pub fn raw_record(
    start_millis: Millisecond,
    end_millis: Millisecond,
) -> Result<Vec<FocusRecord>, String> {
    Ok(read_by_timestamp(start_millis, end_millis))
}

#[tauri::command]
pub fn duration_by_id(
    start_millis: Millisecond,
    end_millis: Millisecond,
) -> Result<HashMap<usize, Millisecond>, String> {
    let records = read_by_timestamp(start_millis, end_millis);
    Ok(group_by_id(records.into_iter().collect()))
}

/// Calculates the duration per day given a time range and timezone offset.
///
/// # Parameters
/// - `start`: The start timestamp in milliseconds.
/// - `end`: The end timestamp in milliseconds.
/// - `time_zone_offset`: The timezone offset in milliseconds.
///
/// # Returns
/// A `Result` containing either:
/// - A `HashMap` where keys are days (in i64 format) and values are durations in milliseconds for that day.
/// - A `String` containing an error message if an error occurs.
#[tauri::command]
pub fn duration_by_day(
    start_millis: Millisecond,
    end_millis: Millisecond,
    time_zone_offset: Millisecond,
) -> Result<HashMap<i64, Millisecond>, String> {
    let records = read_by_timestamp(start_millis, end_millis);
    info!(
        "records len: {:?}, start: {:?}, end: {:?}",
        records.len(),
        start_millis,
        end_millis
    );
    Ok(group_by_day(records, time_zone_offset))
}

#[tauri::command]
pub fn duration_by_day_id(
    start_millis: Millisecond,
    end_millis: Millisecond,
    time_zone_offset: Millisecond,
) -> Result<HashMap<i64, BTreeMap<AppId, Millisecond>>, String> {
    let records = read_by_timestamp(start_millis, end_millis);
    info!(
        "records len: {:?}, start: {:?}, end: {:?}",
        records.len(),
        start_millis,
        end_millis
    );
    Ok(group_by_day_id(records, time_zone_offset))
}

/// Retrieves detailed information about an exe file given its application ID.
///
/// # Parameters
/// - `id`: The application ID for which to retrieve the file details.
///
/// # Returns
/// A `Result` containing either:
/// - A `FileDetail` struct with the file's name, id, path, icon, and version information.
/// - A `String` containing an error message if the file details cannot be retrieved.
#[tauri::command]
pub async fn file_detail(id: usize) -> Result<FileDetail, String> {
    let path = alpha::focus_app::get_path_by_id(id);
    if !Path::new(&path).exists() {
        return Ok(FileDetail {
            name: Path::new(&path)
                .file_name()
                .map(|x| x.to_str().map(|x| x.to_owned()))
                .flatten()
                .unwrap_or(path.to_owned()),
            id,
            path,
            exist: false,
            icon: None,
            version: None,
        });
    }
    let version = util::get_file_version(&path);
    let icon = util::extract_icon(&path).map(|x| {
        let mut buf = Vec::new();
        x.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .unwrap();
        format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(&buf)
        )
    });
    let name = version
        .as_ref()
        .map(|x| x.product_name.to_owned())
        .flatten()
        .or(Path::new(&path)
            .file_name()
            .map(|x| x.to_str().map(|x| x.to_owned()))
            .flatten())
        .unwrap_or(path.to_owned());
    log::debug!("file path: {}", &path);
    Ok(FileDetail {
        name,
        id,
        path,
        exist: true,
        icon,
        version,
    })
}

#[tauri::command]
pub async fn show_in_folder(path: String) -> Result<(), String> {
    Ok(util::show_in_folder(path))
}

#[tauri::command]
pub async fn app_meta() -> Result<AppMeta, String> {
    Ok(alpha::get_app_meta())
}

#[tauri::command]
pub async fn focus_index_record() -> Result<Vec<focus_index::FileIndexRecord>, String> {
    Ok(focus_index::all_record())
}
