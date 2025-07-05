mod cursor_position;
mod focus_event;
pub mod focus_record;
mod millisecond;

pub use cursor_position::CursorPosition;
pub use focus_event::FocusEvent;
pub use millisecond::Millisecond;
use serde::{Deserialize, Serialize};

pub type AppId = usize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMeta {
    pub(crate) start_ms_epoch: u64,
}
