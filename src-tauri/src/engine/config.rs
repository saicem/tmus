use crate::engine::util::{m_as_ms, Timestamp};
use std::time::Duration;

/// If foreground change event interval above this threshold, it's invalid.
pub const INVALID_INTERVAL_BOUND: Timestamp = m_as_ms(3);

/// Check the current window every 1 minute
pub const LOOP_GET_CURRENT_WINDOW_INTERVAL: Duration = Duration::from_secs(60);
