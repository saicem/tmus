use serde::{Deserialize, Serialize};

use crate::util::FileVersion;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub id: usize,
    pub name: String,
    pub path: String,
    pub exist: bool,
    pub icon: Option<String>,
    pub version: Option<FileVersion>,
}
