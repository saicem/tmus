use std::{
    fmt::Debug,
    path::PathBuf,
    sync::{
        mpsc::{channel, Sender},
        OnceLock,
    },
    thread, usize,
};

use log;

use crate::engine::data::{EngineState, FocusEvent, Millisecond};
use crate::engine::file_record::FileRecord;
use super::{
    data::{AppId, EngineError, OptimizeStorageConfig, RecordsQuery},
    file_app::FileApp,
    file_index::{FileIndex, IndexValue},
    Engine, FocusRecord,
};

pub static ENGINE: OnceLock<PigeonEngine> = OnceLock::new();
static SENDER: OnceLock<Sender<FocusEvent>> = OnceLock::new();


/// This engine is designed to allow for accuracy loss.
/// The record with duration less than threshold (set in start function) will be discard.
/// So it's not a good idea to use this engine for accurate recoding of app switching.
/// It's just design for recording the duration of app usage with high efficiency and low storage usage.
pub struct PigeonEngine {
    file_app: FileApp,
    file_index: FileIndex,
    file_record: FileRecord,
    status: EngineState,
}

impl Engine for PigeonEngine {
    fn new(data_dir: &PathBuf) -> PigeonEngine {
        Self {
            file_app: FileApp::new(data_dir),
            file_index: FileIndex::new(data_dir),
            file_record: FileRecord::new(data_dir),
            status: EngineState::Running,
        }
    }

    fn suspend(&mut self) {
        self.status = EngineState::Suspended;
    }

    fn resume(&mut self) {
        self.status = EngineState::Running;
    }

    fn app_path(&self, id: AppId) -> Result<String, EngineError> {
        Ok(self.file_app.get_path_by_id(id).to_owned())
    }

    fn query_records(&self, query: RecordsQuery) -> Result<String, Vec<FocusRecord>> {
        todo!()
    }

    fn optimize_storage(&mut self, config: OptimizeStorageConfig) {
        self.suspend();
        let _ = config;
        self.resume();
    }

    fn on_focus(&self, process_path: &str) {
        match &self.status {
            EngineState::Running => {
                self.add_record_to_queue(process_path)
            }
            EngineState::Busy => {
                self.add_record_to_queue(process_path)
            }
            EngineState::Suspended => {
                log::info!("Receive focus event when suspended, process path: {}", process_path);
            }
        }
    }

    fn read_reverse(&self, cursor: Option<u64>, count: u64) -> (Vec<FocusRecord>, u64) {
        let (records, ret_cursor) = self.file_record.read_reverse(cursor, count);
        (records.into_iter().map(|x| x.into()).collect(), ret_cursor)
    }

    fn write_record(&self, process_path: &str, focus_at: Millisecond, blur_at: Millisecond) {
        let app_id = self.get_id_by_path(&process_path);
        let record = FocusRecord {
            id: app_id,
            focus_at,
            blur_at,
        };
        for sub_record in record.split_record() {
            let index = self.file_record.write(sub_record.unsafe_to_byte());
            self.file_index
                .update_index(sub_record.focus_at.as_days(), index as i64)
        }
    }
}

impl Debug for PigeonEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine").finish()
    }
}

impl PigeonEngine {
    pub fn get_id_by_path(&self, path: &str) -> AppId {
        self.file_app.get_id_by_path(path)
    }

    /// Read records. Both start day and end day are included.
    /// The records may beyond the required range. Need cropped.
    pub fn read_rough_records(&self, start_day: i64, end_day: i64) -> Vec<FocusRecord> {
        let start_index: IndexValue = self.file_index.query_index(start_day).offset(-1);
        let end_index = self.file_index.query_index(end_day + 1);
        log::info!(
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
        }.into_iter()
            .map(|x| x.into())
            .collect()
    }

    fn add_record_to_queue(&self, process_path: &str) {
        let focus_at = Millisecond::now();
        SENDER
            .get()
            .unwrap()
            .send(FocusEvent { app_path: process_path.to_owned(), focus_at })
            .unwrap();
    }
}


/// Can only call once. Return sender, use for sending windows foreground change event.
pub fn start(data_dir: &PathBuf) {
    const THRESHOLD: Millisecond = Millisecond::from_secs(1);
    let channel = channel::<FocusEvent>();
    let (sender, receiver) = channel;
    SENDER.set(sender).unwrap();
    let engine = PigeonEngine::new(data_dir);
    ENGINE.set(engine).expect("Engine init failed.");
    thread::spawn(move || {
        let mut last_focus = FocusEvent {
            app_path: String::default(),
            focus_at: Millisecond::MAX,
        };
        loop {
            let cur_focus = receiver.recv().unwrap();
            log::info!("Receive focus event, last: {:?}, current: {:?}", last_focus, cur_focus);
            if cur_focus.app_path == last_focus.app_path {
                continue;
            }
            // Only record beyond threshold can be storied.
            if last_focus.app_path != String::default() && cur_focus.focus_at - THRESHOLD > last_focus.focus_at {
                ENGINE.get().unwrap().write_record(&last_focus.app_path, last_focus.focus_at, cur_focus.focus_at);
            }
            last_focus = cur_focus;
        }
    });
}