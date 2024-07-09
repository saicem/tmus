use std::{
    fmt::Debug,
    path::PathBuf,
    sync::{
        mpsc::{channel, Sender},
        OnceLock,
    },
    thread,
};

use log::info;

use crate::engine::millisecond::Millisecond;

use super::{
    file_app::FileApp,
    file_index::{FileIndex, IndexValue},
    file_record::FileRecord,
    FocusRecord,
};

pub static ENGINE: OnceLock<Engine> = OnceLock::new();
static SENDER: OnceLock<Sender<FocusEvent>> = OnceLock::new();

/// This engine is designed to allow for accuracy loss.
/// The record with duration less than threshold (set in start function) will be discard.
/// So it's not a good idea to use this engine for accurate recoding of app switching.
/// It's just design for recording the duration of app usage with high efficiency and low storage usage.
pub struct Engine {
    file_app: FileApp,
    file_index: FileIndex,
    file_record: FileRecord,
}

struct FocusEvent {
    app_id: u16,
    focus_at: Millisecond,
}

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine").finish()
    }
}

impl Engine {
    fn new(data_dir: &PathBuf) -> Engine {
        Self {
            file_app: FileApp::new(data_dir),
            file_index: FileIndex::new(data_dir),
            file_record: FileRecord::new(data_dir),
        }
    }

    pub fn get_path_by_id(&self, id: usize) -> String {
        self.file_app.get_path_by_id(id).to_owned()
    }

    pub fn get_id_by_path(&self, path: &str) -> usize {
        self.file_app.get_id_by_path(path)
    }

    /// Read records. Both start day and end day are included.
    /// The records may beyond the required range. Need to be cropped.
    pub fn read_rough_records(&self, start_day: i64, end_day: i64) -> Vec<FocusRecord> {
        let start_index: IndexValue = self.file_index.query_index(start_day).offset(-1);
        let end_index = self.file_index.query_index(end_day + 1);
        info!(
            "read_rough_records start_day:{:?}. end_day{:?}, start_index{:?}, end_index{:?}",
            start_day, end_day, start_index, end_index
        );
        match (start_index, end_index) {
            (_, IndexValue::Before) => vec![],
            (IndexValue::After, _) => vec![],
            (IndexValue::Before, IndexValue::In(end)) => self.file_record.read(0, end as u64),
            (IndexValue::Before, IndexValue::After) => self.file_record.read_to_end(0),
            (IndexValue::In(start), IndexValue::In(end)) => {
                self.file_record.read(start as u64, end as u64)
            }
            (IndexValue::In(start), IndexValue::After) => {
                self.file_record.read_to_end(start as u64)
            }
        }
        .into_iter()
        .map(|x| x.into())
        .collect()
    }

    pub fn read_reverse(&self, cursor: Option<u64>, count: u64) -> (Vec<FocusRecord>, u64) {
        let (records, ret_cursor) = self.file_record.read_reverse(cursor, count);
        (records.into_iter().map(|x| x.into()).collect(), ret_cursor)
    }

    pub fn write_record(&self, record: FocusRecord) {
        for sub_record in record.split_record() {
            let index = self.file_record.write(sub_record.unsafe_to_byte());
            self.file_index
                .update_index(sub_record.focus_at.as_days(), index as i64)
        }
    }
}

/// Can only call once. Return sender, use for sending windows foreground change event.
pub fn start(data_dir: &PathBuf) {
    const THRESHOLD: Millisecond = Millisecond::from_secs(1);
    let (sender, receiver) = channel::<FocusEvent>();
    SENDER.set(sender).unwrap();
    let engine = Engine::new(data_dir);
    ENGINE.set(engine).expect("Engine init failed.");
    thread::spawn(move || {
        let mut last_focus = FocusEvent {
            app_id: u16::MAX,
            focus_at: Millisecond::from_millis(i64::MAX),
        };
        loop {
            let cur_focus = receiver.recv().unwrap();
            info!(
                "last: {:?}, {:?} recv: {:?}, {:?}",
                last_focus.app_id, last_focus.focus_at, cur_focus.app_id, cur_focus.focus_at
            );
            if cur_focus.app_id == last_focus.app_id {
                continue;
            }
            info!("{} {}", last_focus.focus_at, cur_focus.focus_at);
            // Only record beyond threshold can be storied.
            if cur_focus.focus_at - THRESHOLD > last_focus.focus_at {
                ENGINE.get().unwrap().write_record(FocusRecord {
                    id: last_focus.app_id as usize,
                    focus_at: last_focus.focus_at,
                    blur_at: cur_focus.focus_at,
                });
            }
            last_focus = cur_focus;
        }
    });
}

pub fn on_focus(process_path: &str) {
    let focus_at = Millisecond::now();
    let app_id = ENGINE.get().unwrap().get_id_by_path(&process_path) as u16;
    SENDER
        .get()
        .unwrap()
        .send(FocusEvent { app_id, focus_at })
        .unwrap();
}
