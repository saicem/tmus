pub mod focus_app;
pub mod focus_index;
pub mod focus_record;
mod lock;

use super::models;
use crate::models::{CursorPosition, EngineMeta, FocusRecord};
use crate::storage::focus_app::get_id_by_path;
use crate::tracker::TrackingSpanEvent;
use crate::util::{Timestamp, d_as_ms, ms_as_d, s_as_ms};
use crate::EngineError;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

pub use lock::WorkingLock;

const FILE_SHARE_READ: u32 = 1u32;

/// Held for the process lifetime while a working-mode engine is running, so the
/// OS keeps the exclusive lock on the data directory.
static WORKING_LOCK: OnceLock<WorkingLock> = OnceLock::new();

/// Start the storage in working mode: acquire the exclusive data lock first,
/// then initialize the writable storage. Fails with
/// [`EngineError::AlreadyRunning`] when another working-mode engine holds the
/// lock, preventing write conflicts or file deadlocks.
pub fn start_working(data_dir: impl AsRef<Path>) -> Result<(), EngineError> {
    let data_dir = data_dir.as_ref();
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir)?;
    }
    let lock = WorkingLock::acquire(data_dir)?;
    focus_app::init(data_dir);
    focus_index::init(data_dir);
    focus_record::init(data_dir);
    WORKING_LOCK
        .set(lock)
        .map_err(|_| EngineError::AlreadyRunning)?;
    Ok(())
}

/// Start the storage in read-only mode: initialize the read-only storage
/// without acquiring the lock or creating any directory. Missing files are
/// treated as empty data.
pub fn start_readonly(data_dir: impl AsRef<Path>) {
    focus_app::init_readonly(&data_dir);
    focus_index::init_readonly(&data_dir);
    focus_record::init_readonly(&data_dir);
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

pub fn write_record(event: TrackingSpanEvent) {
    if event.name == String::default() || event.blur_at - event.focus_at < s_as_ms(1) {
        return;
    }

    let app_id = get_id_by_path(&event.name);
    let record = FocusRecord {
        id: app_id,
        focus_at: event.focus_at,
        blur_at: event.blur_at,
    };

    for sub_record in record.split_record() {
        let index = focus_record::write(sub_record.unsafe_to_byte());
        focus_index::update_index(ms_as_d(sub_record.focus_at) as u64, index)
    }
}

pub fn get_tmus_meta() -> EngineMeta {
    EngineMeta {
        initial_timestamp: d_as_ms(focus_index::start_day() as i64),
        engine_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("tmus-engine-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn working_lock_is_exclusive_and_reusable() {
        let dir = temp_dir("lock");

        let lock1 = WorkingLock::acquire(&dir).expect("first acquire should succeed");
        let err = WorkingLock::acquire(&dir).expect_err("second acquire must fail");

        assert!(matches!(err, EngineError::AlreadyRunning));

        drop(lock1);
        // After the handle is released the lock can be acquired again, which
        // mirrors a crashed working engine leaving a stale lock file behind.
        WorkingLock::acquire(&dir).expect("acquire after release should succeed");
    }

    #[test]
    fn start_working_is_exclusive() {
        let dir = temp_dir("working");
        start_working(&dir).expect("first start should succeed");
        let err = start_working(&dir).expect_err("second start must fail");
        assert!(matches!(err, EngineError::AlreadyRunning));
    }
}
