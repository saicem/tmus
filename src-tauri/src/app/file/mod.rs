pub mod analyze;
pub mod core;
mod file_app;
mod file_index;
mod file_record;
mod time;

pub use file_record::FocusRecord;

pub use time::custom_now;
