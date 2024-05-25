pub use focus_record::FocusRecord;
use serde::Deserialize;
use serde::Serialize;
pub use tmus_tick::millis_to_tick;
pub use tmus_tick::tick_to_millis;
pub use tmus_tick::TmusTick;
pub use tmus_tick::DAY_TICK;

use super::file_version::FileVersion;

mod focus_record;
mod tmus_tick;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub name: String,
    pub id: usize,
    pub path: String,
    pub version: Option<FileVersion>,
}
