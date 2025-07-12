pub mod async_runtime;
pub mod models;
pub mod storage;
mod tracker;
pub mod util;

use crate::storage::write_record;
use std::path::Path;
use tracing::debug;

pub fn engine_start(data_dir: impl AsRef<Path>, filter: fn(&str) -> Option<String>) {
    storage::init(data_dir);
    tracker::start_tracking(move |tracking_span_event| {
        if let Some(_) = filter(&tracking_span_event.name) {
            write_record(tracking_span_event);
        } else {
            debug!("App is filtered out. {:?}", tracking_span_event);
        }
    });
}
