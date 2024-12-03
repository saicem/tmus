pub mod data;
mod file_app;
mod file_index;
mod focus_record;
mod monitor;
mod engine;
mod file_record;

pub use focus_record::FocusRecord;
pub use engine::ENGINE;
use std::path::PathBuf;

pub fn init(data_dir: &PathBuf) {
    engine::start(data_dir);
    monitor::set_event_hook();
}
