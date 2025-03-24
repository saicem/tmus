use super::data::focus_record::FocusRecord;
use super::data::EngineError;
use super::file_app::FileApp;
use super::file_index::FileIndex;
use crate::engine::alpha::file_record::FileRecord;
use crate::engine::core::{Engine, FocusRecordRaw};
use crate::engine::data::{AppId, CursorPosition, Millisecond};
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::RwLock;

/// This engine is designed to allow for accuracy loss.
/// The record with duration less than threshold (set in start function) will be discarded.
/// So it's not a good idea to use this engine for accurate recoding of app switching.
/// It's just design for recording the duration of app usage with high efficiency and low storage usage.
pub struct AlphaEngine {
    file_app: FileApp,
    file_index: FileIndex,
    file_record: RwLock<FileRecord>,
}

impl Debug for AlphaEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine").finish()
    }
}

impl Engine for AlphaEngine {
    fn new(data_dir: &PathBuf) -> AlphaEngine {
        Self {
            file_app: FileApp::new(data_dir),
            file_index: FileIndex::new(data_dir),
            file_record: RwLock::new(FileRecord::new(data_dir)),
        }
    }

    /// Read records. Include records which blur_at >= start and focus_at <= end,
    /// which means if only need records focus_at >= start and blur_at <= end,
    /// you need to crop the return data.
    fn read_by_time(&self, start: Millisecond, end: Millisecond) -> Vec<FocusRecord> {
        let start_index: CursorPosition = self.file_index.query_index(start.as_days() as u64);
        let end_index = self.file_index.query_index((end.as_days() + 1) as u64);
        match (start_index, end_index) {
            (_, CursorPosition::Start) => vec![],
            (CursorPosition::End, _) => vec![],
            (CursorPosition::Start, CursorPosition::Middle(end)) => {
                self.file_record.read().unwrap().read(0, end)
            }
            (CursorPosition::Start, CursorPosition::End) => {
                self.file_record.read().unwrap().read_to_end(0)
            }
            (CursorPosition::Middle(start), CursorPosition::Middle(end)) => {
                self.file_record.read().unwrap().read(start, end)
            }
            (CursorPosition::Middle(start), CursorPosition::End) => {
                self.file_record.read().unwrap().read_to_end(start)
            }
        }
        .into_iter()
        .map(|x| x.into())
        .collect()
    }

    fn get_id_by_path(&self, path: &str) -> AppId {
        self.file_app.get_id_by_path(path)
    }

    fn get_path_by_id(&self, id: AppId) -> Result<String, EngineError> {
        Ok(self.file_app.get_path_by_id(id).to_owned())
    }

    fn get_all_app(&self) -> Vec<(AppId, String)> {
        self.file_app
            .get_all_app()
            .into_iter()
            .enumerate()
            .map(|(id, path)| (id as AppId, path))
            .collect()
    }

    fn write_record(&self, raw: FocusRecordRaw) {
        let FocusRecordRaw {
            app_path,
            focus_at,
            blur_at,
        } = raw;
        if app_path == String::default() || blur_at - focus_at < Millisecond::ONE_SECOND {
            return;
        }

        let app_id = self.get_id_by_path(&app_path);
        let record = FocusRecord {
            id: app_id,
            focus_at,
            blur_at,
        };

        for sub_record in record.split_record() {
            let index = self
                .file_record
                .write()
                .unwrap()
                .write(sub_record.unsafe_to_byte());
            self.file_index
                .update_index(sub_record.focus_at.as_days() as u64, index)
        }
    }
}
