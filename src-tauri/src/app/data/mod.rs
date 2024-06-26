pub use focus_record::FocusRecord;
use serde::Deserialize;
use serde::Serialize;
pub use tick::Tick;

use super::file_version::FileVersion;

mod focus_record;
mod tick;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub id: usize,
    pub name: String,
    pub path: String,
    pub version: Option<FileVersion>,
}
