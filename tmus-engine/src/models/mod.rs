mod focus_record;

use crate::util::Timestamp;
use serde::{Deserialize, Serialize};

pub type AppId = usize;

pub use focus_record::FocusRecord;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineMeta {
    pub initial_timestamp: Timestamp,
    pub engine_version: String,
}

pub(crate) use focus_record::RecordByte;

#[derive(Debug, PartialEq)]
pub(crate) enum CursorPosition {
    Start,
    End,
    Middle(usize),
}
