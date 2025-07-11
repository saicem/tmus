mod monitor;

use crate::util::Timestamp;
use std::time::Duration;

pub use monitor::set_window_tracker;

#[derive(Debug)]
pub struct WindowFocusEvent {
    /// The path of the executable of the focused window.
    pub app_path: String,
    /// The time when the window gained focus.
    pub focus_at: Timestamp,
}

pub struct WindowTrackerConfig {
    /// Check the current window interval.
    loop_get_current_window_interval: Duration,
}

impl Default for WindowTrackerConfig {
    fn default() -> Self {
        Self {
            loop_get_current_window_interval: Duration::from_secs(60),
        }
    }
}
