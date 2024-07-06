use std::{
    fmt::Debug,
    path::PathBuf,
    sync::{
        mpsc::{channel, Sender},
        OnceLock,
    },
    thread,
};

use crate::engine::r#type::{AppId, Millisecond};

use super::{
    file_app::FileApp,
    file_index::{FileIndex, IndexValue},
    file_record::FileRecord,
    focus_record::SAFE_SPAN,
    now,
    r#type::Day,
    FocusRecord,
};

pub static ENGINE: OnceLock<Engine> = OnceLock::new();

/// This engine is designed to allow for accuracy loss.
/// The record with duration less than threshold (set in start function) will be discard.
/// So it's not a good idea to use this engine for accurate recoding of app switching.
/// It's just design for recording the duration of app usage with high efficiency and low storage usage.
pub struct Engine {
    file_app: FileApp,
    file_index: FileIndex,
    file_record: FileRecord<8>,
    sender: Sender<FocusEvent>,
}

struct FocusEvent {
    app_id: u16,

    /// Seconds from unix epoch.
    focus_at: u32,
}

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine").finish()
    }
}

impl Engine {
    fn new(data_dir: &PathBuf, sender: Sender<FocusEvent>) -> Engine {
        Self {
            file_app: FileApp::new(data_dir),
            file_index: FileIndex::new(data_dir),
            file_record: FileRecord::new(data_dir),
            sender,
        }
    }

    pub fn get_path_by_id(&self, id: usize) -> String {
        self.file_app.get_path_by_id(id).to_owned()
    }

    /// Read records. Both start day and end day are included.
    /// The records may beyond the required range. Need to be cropped.
    pub fn read_rough_records(&self, start: Day, end: Day) -> Vec<FocusRecord> {
        let start_index = self.file_index.query_index(start).offset(-SAFE_SPAN);
        let end_index = self.file_index.query_index(end + 1).offset(SAFE_SPAN);
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

    pub fn write_record(&self, record: FocusRecord) {
        for sub_record in record.split_record() {
            let index = self.file_record.write(sub_record.unsafe_to_byte());
            self.file_index.update_index(sub_record.focus_at, index as i64)
        }
    }

    pub fn on_focus(&self, process_path: &str) {
        let focus_at = now().as_secs() as u32;
        let app_id = self.file_app.get_id_by_path(&process_path) as u16;
        self.sender.send(FocusEvent { app_id, focus_at }).unwrap();
    }
}

/// Can only call once. Return sender, use for sending windows foreground change event.
pub fn start(data_dir: &PathBuf) {
    let threshold = 1;
    let (sender, receiver) = channel::<FocusEvent>();
    let engine = Engine::new(data_dir, sender);
    ENGINE.set(engine).expect("Engine init failed.");
    thread::spawn(move || {
        let mut last_focus = FocusEvent {
            app_id: u16::MAX,
            focus_at: u32::MAX,
        };
        loop {
            let cur_focus = receiver.recv().unwrap();
            println!("recv: {}, {}", cur_focus.app_id, cur_focus.focus_at);
            // Only record beyond threshold can be storied.
            if cur_focus.app_id != last_focus.app_id
                && cur_focus.focus_at - threshold > last_focus.focus_at
            {
                ENGINE.get().unwrap().write_record(FocusRecord {
                    id: last_focus.app_id as AppId,
                    focus_at: last_focus.focus_at as Millisecond * 1000,
                    blur_at: cur_focus.focus_at as Millisecond * 1000,
                });
            }
            last_focus = cur_focus;
        }
    });
}
