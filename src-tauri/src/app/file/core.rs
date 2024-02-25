use crate::app::file::file_app::AppTxtFile;
use crate::app::file::file_index::IndexBinFile;
use crate::app::file::file_record::RecordBinFile;
use crate::app::file::time::DAY_CENTI_SEC;
use crate::app::file::FocusRecord;
use std::cell::OnceCell;
use std::path::PathBuf;

static mut FILE_APP: OnceCell<AppTxtFile> = OnceCell::new();
static mut FILE_INDEX: OnceCell<IndexBinFile> = OnceCell::new();
static mut FILE_RECORD: OnceCell<RecordBinFile> = OnceCell::new();

pub unsafe fn init(data_dir: &PathBuf) {
    FILE_APP.set(AppTxtFile::new(&data_dir));
    FILE_INDEX.set(IndexBinFile::new(&data_dir));
    FILE_RECORD.set(RecordBinFile::new(&data_dir));
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

pub unsafe fn get_app_name_by_id(id: u64) -> String {
    FILE_APP.get_mut().unwrap().get_name_by_id(id)
}

pub unsafe fn get_app_id_by_name(name: &str) -> u64 {
    FILE_APP.get_mut().unwrap().get_id_by_name(name)
}
