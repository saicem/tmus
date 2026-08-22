use crate::EngineError;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

/// An exclusive lock on the data directory held by a working-mode engine.
///
/// The lock is implemented as a lock file opened with no share mode
/// (`share_mode(0)`): the OS refuses a second open while the first handle is
/// alive, and the lock is released automatically when the process exits or
/// crashes. A leftover lock file is harmless and never blocks re-acquisition.
#[derive(Debug)]
pub struct WorkingLock {
    /// Held for the process lifetime so the OS keeps the exclusive share mode.
    _file: File,
    _path: PathBuf,
}

impl WorkingLock {
    /// Acquire the exclusive lock, or return [`EngineError::AlreadyRunning`]
    /// when another working-mode engine holds it.
    pub fn acquire(data_dir: &Path) -> Result<WorkingLock, EngineError> {
        let path = data_dir.join("engine.lock");
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .share_mode(0)
            .open(&path)
            .map_err(|e| {
                if is_busy_error(&e) {
                    EngineError::AlreadyRunning
                } else {
                    EngineError::Io(e)
                }
            })?;
        Ok(WorkingLock {
            _file: file,
            _path: path,
        })
    }
}

/// Windows treats a busy file as a sharing/lock violation (5, 32, 33) and the
/// standard library may map it to `Uncategorized`, so check the raw OS error
/// in addition to `PermissionDenied`.
fn is_busy_error(e: &std::io::Error) -> bool {
    e.kind() == ErrorKind::PermissionDenied || matches!(e.raw_os_error(), Some(5 | 32 | 33))
}
