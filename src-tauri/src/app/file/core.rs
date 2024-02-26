use crate::app::file::file_app::AppTxtFile;
use crate::app::file::file_index::IndexBinFile;
use crate::app::file::file_record::RecordBinFile;
use crate::app::file::time::{custom_datetime, DAY_CENTI_SEC};
use crate::app::file::FocusRecord;
use chrono::{DateTime, Utc};
use std::cell::OnceCell;
use std::path::PathBuf;
use tauri;

static mut FILE_APP: OnceCell<AppTxtFile> = OnceCell::new();
static mut FILE_INDEX: OnceCell<IndexBinFile> = OnceCell::new();
static mut FILE_RECORD: OnceCell<RecordBinFile> = OnceCell::new();

pub unsafe fn init(data_dir: &PathBuf) {
    FILE_APP.set(AppTxtFile::new(&data_dir)).unwrap();
    FILE_INDEX.set(IndexBinFile::new(&data_dir)).unwrap();
    FILE_RECORD.set(RecordBinFile::new(&data_dir)).unwrap();
}

pub unsafe fn write_record(
    app_id: u64,
    start_day: u64,
    mut start_time: u64,
    end_day: u64,
    end_time: u64,
) {
    for _ in start_day..end_day {
        let cur_position = FILE_RECORD.get_mut().unwrap().cur_position();
        FILE_INDEX.get_mut().unwrap().write_index(cur_position);
        FILE_RECORD
            .get_mut()
            .unwrap()
            .write_record(FocusRecord::new(
                app_id,
                start_time,
                DAY_CENTI_SEC - start_time,
            ));
        start_time = 0;
    }
    FILE_RECORD
        .get_mut()
        .unwrap()
        .write_record(FocusRecord::new(app_id, start_time, end_time))
}

#[tauri::command]
pub unsafe fn read_record_by_index(start: u64, end: u64) -> Vec<FocusRecord> {
    FILE_RECORD.get_mut().unwrap().read_record(start, end)
}

/// Query records between start day and end day.
/// `start_day` must be less than `end_day`.
#[tauri::command]
pub unsafe fn read_records_by_day(start_day: u64, end_day: u64) -> Vec<FocusRecord> {
    let (mut start_index, mut end_index) = FILE_INDEX
        .get_mut()
        .unwrap()
        .query_index(start_day, end_day);
    if start_index == u64::MAX {
        return Vec::new();
    }

    let file_app = FILE_RECORD.get_mut().unwrap();
    if end_index == u64::MAX {
        return file_app.read_to_end(start_index);
    }
    file_app.read_record(start_index, end_index)
}

pub unsafe fn read_records_by_datetime(start_datetime: DateTime<Utc>, end_datetime: DateTime<Utc>) {
    let (start_day, start_time) = custom_datetime(start_datetime);
    let (end_day, end_time) = custom_datetime(end_datetime);
    let records = read_records_by_day(start_day, end_day);

    // TODO Implement an algorithm for finding indexes for records between start and end times
    let find = || {};
}

pub unsafe fn get_app_name_by_id(id: u64) -> String {
    FILE_APP.get_mut().unwrap().get_name_by_id(id)
}

pub unsafe fn get_app_id_by_name(name: &str) -> u64 {
    FILE_APP.get_mut().unwrap().get_id_by_name(name)
}
