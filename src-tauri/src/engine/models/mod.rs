pub mod focus_record;

use crate::engine::util::Timestamp;
use serde::{Deserialize, Serialize};

pub type AppId = usize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMeta {
    pub(crate) initial_timestamp: Timestamp,
    pub(crate) tmus_version: String,
}

#[derive(Debug, PartialEq)]
pub enum CursorPosition {
    Start,
    End,
    Middle(usize),
}

/// An event that represents a window gaining focus.
///
/// It is used to pass information between the thread that listens for focus events and the thread that records the focus
/// events.
#[derive(Debug)]
pub struct FocusEvent {
    /// The path of the executable of the focused window.
    pub(crate) app_path: String,
    /// The time when the window gained focus.
    pub(crate) focus_at: Timestamp,
}
