pub use engine::ENGINE;
pub use focus_record::FocusRecord;
pub use millisecond::Millisecond;
use std::{
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod engine;
mod file_app;
mod file_index;
mod file_record;
mod focus_record;
mod monitor;
mod millisecond;

pub fn init(data_dir: &PathBuf) {
    engine::start(data_dir);
    monitor::set_event_hook();
}
