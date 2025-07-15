pub mod async_runtime;
pub mod models;
pub mod storage;
mod tracker;
pub mod util;

use crate::storage::write_record;
use crate::tracker::TrackingSpanEvent;
use std::path::Path;
use tracing::debug;

pub fn engine_start(data_dir: impl AsRef<Path>, filter: fn(&str) -> Option<String>) {
    storage::init(data_dir);
    tracker::start_tracking(move |tracking_span_event| {
        if let Some(new_name) = filter(&tracking_span_event.name) {
            write_record(TrackingSpanEvent {
                name: new_name,
                focus_at: tracking_span_event.focus_at,
                blur_at: tracking_span_event.blur_at,
            });
        } else {
            debug!("App is filtered out. {:?}", tracking_span_event);
        }
    });
}
