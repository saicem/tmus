mod cursor_position;
mod engine_error;
mod focus_event;
pub mod focus_record;
mod millisecond;

use serde::{Deserialize, Serialize};
pub use cursor_position::CursorPosition;
pub use engine_error::EngineError;
pub use focus_event::FocusEvent;
pub use millisecond::Millisecond;

pub type AppId = usize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMeta {
    pub(crate) start_ms_epoch: u64,
}
