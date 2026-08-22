use serde::Serialize;
use std::fmt;

/// Unified error type returned by Tauri commands.
///
/// `code` is a stable machine-readable identifier the frontend can use to map
/// to localized messages; `message` is a human-readable detail (kept as a
/// fallback until the frontend mapping table is complete).
#[derive(Debug, Serialize)]
pub struct TmusError {
    pub code: String,
    pub message: String,
}

impl TmusError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    /// Errors from business logic that carry no dedicated code yet.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("internal", message)
    }
}

impl fmt::Display for TmusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TmusError {}

impl From<String> for TmusError {
    fn from(value: String) -> Self {
        Self::internal(value)
    }
}

impl From<&str> for TmusError {
    fn from(value: &str) -> Self {
        Self::internal(value)
    }
}
