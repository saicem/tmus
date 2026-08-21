pub mod async_runtime;
pub mod models;
pub mod storage;
mod tracker;
pub mod util;

use crate::storage::write_record;
use crate::tracker::TrackingSpanEvent;
use crate::util::Timestamp;
use std::fmt;
use std::path::Path;
use std::sync::OnceLock;
use tracing::debug;

/// Engine startup mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineMode {
    /// Listening and writing mode. Acquires an exclusive lock on the data
    /// directory and starts the window tracker. Fails with
    /// [`EngineError::AlreadyRunning`] if another working-mode engine is
    /// already running, to avoid write conflicts and file deadlocks.
    Working,
    /// Read-only mode. Never acquires the lock, never writes data, and never
    /// starts the tracker. Used by short-lived CLI processes so they can query
    /// data while a working-mode engine is running.
    ReadOnly,
}

/// Errors returned when starting the engine.
#[derive(Debug)]
pub enum EngineError {
    /// Another working-mode engine holds the exclusive data lock.
    AlreadyRunning,
    /// The engine has already been started in this process. A process can use
    /// exactly one mode for its whole lifetime.
    AlreadyStarted,
    /// Underlying IO failure.
    Io(std::io::Error),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::AlreadyRunning => {
                write!(f, "Another working-mode engine instance is already running")
            }
            EngineError::AlreadyStarted => {
                write!(f, "The engine has already been started in this process")
            }
            EngineError::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for EngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EngineError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EngineError {
    fn from(e: std::io::Error) -> Self {
        EngineError::Io(e)
    }
}

static ENGINE_MODE: OnceLock<EngineMode> = OnceLock::new();

/// Start the engine in the given mode.
///
/// * `Working` initializes the writable storage under an exclusive lock and
///   starts the window tracker; the `filter` callback decides which tracked
///   app paths are recorded (or merged) before writing.
/// * `ReadOnly` only initializes the read-only storage and ignores `filter`.
///
/// Returns [`EngineError::AlreadyRunning`] in working mode when another
/// working-mode engine already holds the data lock.
pub fn engine_start(
    data_dir: impl AsRef<Path>,
    mode: EngineMode,
    filter: fn(&str) -> Option<String>,
) -> Result<(), EngineError> {
    if ENGINE_MODE.get().is_some() {
        return Err(EngineError::AlreadyStarted);
    }

    match mode {
        EngineMode::Working => {
            storage::start_working(&data_dir)?;
            tracker::start_tracking(move |tracking_span_event| {
                if let Some(new_name) = filter(&tracking_span_event.name) {
                    debug!(
                        "New record {:?} {:?} {:?} {:?}",
                        new_name,
                        tracking_span_event.focus_at,
                        tracking_span_event.blur_at,
                        format_duration(tracking_span_event.blur_at - tracking_span_event.focus_at)
                    );
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
        EngineMode::ReadOnly => {
            storage::start_readonly(&data_dir);
        }
    }

    ENGINE_MODE
        .set(mode)
        .expect("engine mode was checked before start");
    Ok(())
}

fn format_duration(duration: Timestamp) -> String {
    let seconds = duration / 1000;
    let millis = duration % 1000;
    format!("{}.{:03}", seconds, millis)
}
