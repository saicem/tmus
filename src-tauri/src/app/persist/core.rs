use std::sync::{Mutex, OnceLock};

use tauri;

use crate::app::data::focus_record::FocusRecord;
use crate::app::global;
use crate::app::persist::file_app::AppTxtFile;
use crate::app::persist::file_index::IndexBinFile;
use crate::app::persist::file_record::RecordBinFile;
use crate::app::persist::tmus_tick::{TmusTick, DAY_TICK};

static FILE_APP: OnceLock<Mutex<AppTxtFile>> = OnceLock::new();
pub(crate) static FILE_INDEX: OnceLock<Mutex<IndexBinFile>> = OnceLock::new();
pub(crate) static FILE_RECORD: OnceLock<Mutex<RecordBinFile>> = OnceLock::new();

pub fn init() {
    let data_dir = global::DATA_DIR.get().unwrap();
    FILE_APP
        .set(Mutex::new(AppTxtFile::new(&data_dir)))
        .unwrap();
    FILE_INDEX
        .set(Mutex::new(IndexBinFile::new(&data_dir)))
        .unwrap();
    FILE_RECORD
        .set(Mutex::new(RecordBinFile::new(&data_dir)))
        .unwrap();
}

pub fn write_record(app_id: u64, start: TmusTick, end: TmusTick) {
    let mut start_tick_of_day = start.tick_of_day();
    for _ in start.day()..end.day() {
        let cur_position = FILE_RECORD.get().unwrap().lock().unwrap().cur_position();
        FILE_INDEX
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .write_index(cur_position);
        FILE_RECORD
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .write_record(FocusRecord::new(
                app_id,
                start_tick_of_day,
                DAY_TICK - start_tick_of_day,
            ));
        start_tick_of_day = 0;
    }
    FILE_RECORD
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .write_record(FocusRecord::new(
            app_id,
            start_tick_of_day,
            end.tick_of_day(),
        ))
}

pub fn get_app_name_by_id(id: u64) -> String {
    FILE_APP.get().unwrap().lock().unwrap().get_name_by_id(id)
}

pub fn get_app_id_by_name(name: &str) -> u64 {
    FILE_APP.get().unwrap().lock().unwrap().get_id_by_name(name)
}
