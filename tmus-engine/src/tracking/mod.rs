pub mod focus_app;
pub mod focus_index;
pub mod focus_record;

use super::{models, FocusRecord};
use crate::core::FocusRecordRaw;
use crate::models::{AppMeta, CursorPosition};
use crate::tracking::focus_app::get_id_by_path;
use crate::util::{d_as_ms, ms_as_d, s_as_ms, Timestamp};
use std::path::PathBuf;

pub(crate) fn init(data_dir: &PathBuf) {
    focus_app::init(data_dir);
    focus_index::init(data_dir);
    focus_record::init(data_dir);
}

/// Read records. Include records which blur_at >= start and focus_at <= end,
/// which means if only need records focus_at >= start and blur_at <= end,
/// you need to crop the return data.
pub fn read_by_timestamp(start: Timestamp, end: Timestamp) -> Vec<FocusRecord> {
    let start_index: CursorPosition = focus_index::query_index(ms_as_d(start) as u64);
    let end_index = focus_index::query_index((ms_as_d(end) + 1) as u64);
    if start_index == CursorPosition::End || end_index == CursorPosition::Start {
        return vec![];
    }
    let start = match start_index {
        CursorPosition::Start => None,
        CursorPosition::Middle(start) => Some(start),
        CursorPosition::End => panic!("start_index should not be CursorPosition::End"),
    };
    let end = match end_index {
        CursorPosition::Start => panic!("end_index should not be CursorPosition::Start"),
        CursorPosition::Middle(end) => Some(end),
        CursorPosition::End => None,
    };
    focus_record::read(start, end)
        .into_iter()
        .map(|x| x.into())
        .collect()
}

pub(crate) fn write_record(raw: FocusRecordRaw) {
    let FocusRecordRaw {
        app_path,
        focus_at,
        blur_at,
    } = raw;
    if app_path == String::default() || blur_at - focus_at < s_as_ms(1) {
        return;
    }

    let app_id = get_id_by_path(&app_path);
    let record = FocusRecord {
        id: app_id,
        focus_at,
        blur_at,
    };

    for sub_record in record.split_record() {
        let index = focus_record::write(sub_record.unsafe_to_byte());
        focus_index::update_index(ms_as_d(sub_record.focus_at) as u64, index)
    }
}

pub fn get_tmus_meta() -> AppMeta {
    AppMeta {
        initial_timestamp: d_as_ms(focus_index::start_day() as i64),
        tmus_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}
