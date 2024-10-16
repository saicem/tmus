pub mod data;
mod file_app;
mod file_index;
mod focus_record;
mod interface;
mod monitor;
mod pigeon_engine;
mod file_record;

pub use focus_record::FocusRecord;
pub use interface::Engine;
pub use pigeon_engine::ENGINE;
use std::path::PathBuf;

pub fn init(data_dir: &PathBuf) {
    pigeon_engine::start(data_dir);
    monitor::set_event_hook();
}
