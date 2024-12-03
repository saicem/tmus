use std::path::PathBuf;
use std::fmt::Debug;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::OnceLock;
use std::thread;

use log;

use crate::engine::data::{CursorPosition, EngineState, FocusEvent, Millisecond, ReadDirection};
use crate::engine::file_record::FileRecord;
use super::file_app::FileApp;
use super::data::AppId;
use super::data::EngineError;
use super::data::OptimizeStorageOptions;
use super::file_index::FileIndex;
use super::FocusRecord;

pub static ENGINE: OnceLock<Engine> = OnceLock::new();
static SENDER: OnceLock<Sender<FocusEvent>> = OnceLock::new();


/// This engine is designed to allow for accuracy loss.
/// The record with duration less than threshold (set in start function) will be discarded.
/// So it's not a good idea to use this engine for accurate recoding of app switching.
/// It's just design for recording the duration of app usage with high efficiency and low storage usage.
pub struct Engine {
    file_app: FileApp,
    file_index: FileIndex,
    file_record: FileRecord,
    status: EngineState,
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
            status: EngineState::Running,
        }
    }

    /// Not implement
    /// Suspend engine, the app recording will truncate and set the current time as the end of focus duration.
    fn suspend(&mut self) {
        self.status = EngineState::Suspended;
    }

    /// Not implement
    fn resume(&mut self) {
        self.status = EngineState::Running;
    }

    /// Not implement
    fn optimize_storage(&mut self, config: OptimizeStorageOptions) {
        self.suspend();
        let _ = config;
        self.resume();
    }

    pub(crate) fn on_focus(&self, process_path: &str) {
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

    /// Read records. Include records which blur_at >= start and focus_at <= end,
    /// which means if only need records focus_at >= start and blur_at <= end,
    /// you need to crop the return data.
    pub(crate) fn read_by_time(&self, start: Millisecond, end: Millisecond) -> Vec<FocusRecord> {
        let start_index: CursorPosition = self.file_index.query_index(start.as_days() as u64);
        let end_index = self.file_index.query_index((end.as_days() + 1) as u64);
        match (start_index, end_index) {
            (_, CursorPosition::Start) => vec![],
            (CursorPosition::End, _) => vec![],
            (CursorPosition::Start, CursorPosition::Middle(end)) => self.file_record.read(0, end),
            (CursorPosition::Start, CursorPosition::End) => self.file_record.read_to_end(0),
            (CursorPosition::Middle(start), CursorPosition::Middle(end)) => {
                self.file_record.read(start, end)
            }
            (CursorPosition::Middle(start), CursorPosition::End) => {
                self.file_record.read_to_end(start)
            }
        }.into_iter()
            .map(|x| x.into())
            .collect()
    }

    /// Reads a batch of data starting from the given cursor position in the specified direction.
    ///
    /// # Arguments 
    ///
    /// * `cursor`: An optional cursor position to start reading from. If `None`, starts from the end.
    /// * `count`: The number of records to read.
    /// * `direction`: The direction to read, either forward or backward.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A vector of `FocusRecord` instances.
    /// - A `u64` representing the new cursor position.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example usage
    /// ```
    pub(crate) fn read_by_cursor(&self, cursor: CursorPosition, count: u64, direction: ReadDirection) -> (Vec<FocusRecord>, CursorPosition) {
        let (records, ret_cursor) = self.file_record.read_by_cursor(cursor, count, direction);
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
                .update_index(sub_record.focus_at.as_days() as u64, index)
        }
    }

    pub fn get_id_by_path(&self, path: &str) -> AppId {
        self.file_app.get_id_by_path(path)
    }

    pub fn get_path_by_id(&self, id: AppId) -> Result<String, EngineError> {
        Ok(self.file_app.get_path_by_id(id).to_owned())
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
    let engine = Engine::new(data_dir);
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