use std::path::PathBuf;
use tmus_engine::{EngineError, EngineMode, engine_start};

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tmus-engine-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn readonly_mode_starts_without_lock_and_queries() {
    let dir = temp_dir("it-readonly");

    engine_start(&dir, EngineMode::ReadOnly, |_| None).expect("readonly start should succeed");

    // One process uses exactly one mode for its lifetime.
    let err = engine_start(&dir, EngineMode::Working, |_| None)
        .expect_err("starting another mode in the same process must fail");
    assert!(matches!(err, EngineError::AlreadyStarted));

    // Read-only queries work on empty data without creating any files.
    let records = tmus_engine::storage::read_by_timestamp(0, i64::MAX);
    assert!(records.is_empty());
    assert!(!dir.join("record.bin").exists());
    assert!(!dir.join("engine.lock").exists());
}
