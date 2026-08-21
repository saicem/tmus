use std::path::PathBuf;
use tmus_engine::storage::WorkingLock;
use tmus_engine::{engine_start, EngineError, EngineMode};

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tmus-engine-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn working_mode_requires_exclusive_lock() {
    let dir = temp_dir("it-working");

    // A held lock (as if another working-mode engine is running) must prevent
    // a new working-mode engine from starting.
    let lock = WorkingLock::acquire(&dir).expect("manual lock should succeed");
    let err = engine_start(&dir, EngineMode::Working, |_| None)
        .expect_err("working start must fail while the lock is held");
    assert!(matches!(err, EngineError::AlreadyRunning));

    // After the lock is released, starting succeeds.
    drop(lock);
    engine_start(&dir, EngineMode::Working, |_| None).expect("working start should succeed");

    // One process uses exactly one mode for its lifetime.
    let err = engine_start(&dir, EngineMode::Working, |_| None)
        .expect_err("starting the engine twice must fail");
    assert!(matches!(err, EngineError::AlreadyStarted));
}
