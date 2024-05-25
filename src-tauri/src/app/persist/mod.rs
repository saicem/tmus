pub use file_app::get_id_by_path;
pub use file_app::get_path_by_id;
pub use read::read_records_by_datetime;

use super::data::Tick;
use crate::app::data::FocusRecord;
use crate::app::global;

mod file_app;
mod file_index;
mod file_record;
mod read;

pub fn init() {
    let data_dir = global::DATA_DIR.get().unwrap();
    file_app::init(data_dir);
    file_index::init(data_dir);
    file_record::init(data_dir);
}

pub fn write_record(app_id: u64, start: Tick, end: Tick) {
    let mut start_tick_of_day = start.day_tick();
    for _ in start.day()..end.day() {
        let cur_position = file_record::cur_position();
        file_index::write_index(cur_position);
        file_record::write_record(&FocusRecord::new(
            app_id,
            start_tick_of_day,
            Tick::from_days(1) - start_tick_of_day,
        ));
        start_tick_of_day = Tick(0);
    }
    file_record::write_record(&FocusRecord::new(
        app_id,
        start_tick_of_day,
        end.day_tick() - start_tick_of_day,
    ))
}
