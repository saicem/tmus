pub use engine::ENGINE;
pub use focus_record::FocusRecord;
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
pub mod r#type;

pub fn init(data_dir: &PathBuf) {
    engine::start(data_dir);
    monitor::set_event_hook();
}

fn now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch")
}

fn now_day() -> u64 {
    now().as_secs() / (24 * 60 * 60)
}

// pub fn write_record(app_id: u64, start: Tick, end: Tick) {
//     let mut start_tick_of_day = start.day_tick();
//     for _ in start.day()..end.day() {
//         let cur_position = file_record::cur_position();
//         file_index::write_index(cur_position);
//         file_record::write_record(&FocusRecord::new(
//             app_id,
//             start_tick_of_day,
//             Tick::from_days(1) - start_tick_of_day,
//         ));
//         start_tick_of_day = Tick(0);
//     }
//     file_record::write_record(&FocusRecord::new(
//         app_id,
//         start_tick_of_day,
//         end.day_tick() - start_tick_of_day,
//     ))
// }
