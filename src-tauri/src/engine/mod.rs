pub mod data;
mod engine;
mod file_app;
mod file_index;
mod file_record;
mod monitor;

pub use data::focus_record::FocusRecord;
pub use engine::Engine;
use std::path::PathBuf;

pub fn init(data_dir: &PathBuf) {
    Engine::init(data_dir);
    Engine::start();
    monitor::set_event_hook();
}
