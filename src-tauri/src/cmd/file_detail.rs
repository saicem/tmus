use serde::{Deserialize, Serialize};

use crate::util::file_version::FileVersion;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub id: usize,
    pub name: String,
    pub path: String,
    pub version: Option<FileVersion>,
}
