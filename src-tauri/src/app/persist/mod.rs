pub use file_app::get_id_by_name;
pub use read::read_records_by_datetime;

use crate::app::data::{FocusRecord, TmusTick, DAY_TICK};
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

pub fn write_record(app_id: u64, start: TmusTick, end: TmusTick) {
    let mut start_tick_of_day = start.tick_of_day();
    for _ in start.day()..end.day() {
        let cur_position = file_record::cur_position();
        file_index::write_index(cur_position);
        file_record::write_record(&FocusRecord::new(
            app_id,
            start_tick_of_day,
            DAY_TICK - start_tick_of_day,
        ));
        start_tick_of_day = 0;
    }
    file_record::write_record(&FocusRecord::new(
        app_id,
        start_tick_of_day,
        end.tick_of_day(),
    ))
}
