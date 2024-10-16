use super::{
    data::{AppId, EngineError, OptimizeStorageConfig, RecordsQuery},
    FocusRecord,
};
use std::path::PathBuf;
use crate::engine::data::Millisecond;

pub trait Engine {
    fn new(data_dir: &PathBuf) -> Self;

    /// Suspend engine, the app recording will truncate and set the current time as the end of focus duration.
    fn suspend(&mut self);

    fn resume(&mut self);

    fn app_path(&self, id: AppId) -> Result<String, EngineError>;

    fn query_records(&self, query: RecordsQuery) -> Result<String, Vec<FocusRecord>>;

    fn optimize_storage(&mut self, config: OptimizeStorageConfig);

    fn on_focus(&self, process_path: &str);

    fn read_reverse(&self, cursor: Option<u64>, count: u64) -> (Vec<FocusRecord>, u64);

    fn write_record(&self, process_path: &str, focus_at: Millisecond, blur_at: Millisecond);
}
