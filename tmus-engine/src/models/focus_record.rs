use crate::models::AppId;
use crate::util::{Timestamp, ms_as_d, ms_as_s, s_as_ms, start_of_next_d};
use serde::Serialize;
use std::fmt::Debug;

const DURATION_MAX: Timestamp = s_as_ms(u16::MAX as i64);

pub(crate) type RecordByte = [u8; 8];

/// - app_id: 2^16, 8192 applications support.
/// - focus_at: 2^32, The time when the focus starts. About 136 year from unix epoch.
/// - duration: 2^16, Maximum time that can be represented is about 0.76 day,
#[derive(Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FocusRecord {
    pub id: AppId,
    pub focus_at: Timestamp,
    pub blur_at: Timestamp,
}

impl Debug for FocusRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusRecord")
            .field("id", &self.id)
            .field("focus_at", &self.focus_at)
            .field("blur_at", &self.blur_at)
            .field("duration", &self.duration())
            .finish()
    }
}

impl FocusRecord {
    pub fn duration(&self) -> Timestamp {
        self.blur_at - self.focus_at
    }

    /// Convert to bytes. Use [`Self::split_record`] method ensure that the duration value is safe.
    pub(crate) fn unsafe_to_byte(&self) -> RecordByte {
        let mut ret = RecordByte::default();
        let id = self.id as u16;
        let focus_at = ms_as_s(self.focus_at) as u32;
        let duration = ms_as_s(self.duration()) as u16;
        ret[..2].copy_from_slice(&id.to_le_bytes());
        ret[2..4].copy_from_slice(&duration.to_le_bytes());
        ret[4..].copy_from_slice(&focus_at.to_le_bytes());
        ret
    }

    /// Split record into multiple records.
    /// Ensure that the duration of each record is less than 0.76 day(The maximum time that can be represented)
    /// and not span across one day which could make index easier.
    /// TODO: Optimizing with iterators!
    pub(crate) fn split_record(&self) -> Vec<FocusRecord> {
        assert!(self.blur_at >= self.focus_at);
        self.split_by_not_across_day()
            .iter()
            .map(|x| x.split_by_max_duration())
            .flatten()
            .collect()
    }

    fn new(id: usize, focus_at: Timestamp, blur_at: Timestamp) -> FocusRecord {
        FocusRecord {
            id,
            focus_at,
            blur_at,
        }
    }

    fn from_byte(bytes: RecordByte) -> Self {
        let id = u16::from_le_bytes(bytes[..2].try_into().unwrap()) as usize;
        let focus_at = s_as_ms(u32::from_le_bytes(bytes[4..].try_into().unwrap()) as i64);
        let blur_at =
            focus_at + s_as_ms(u16::from_le_bytes(bytes[2..4].try_into().unwrap()) as i64);
        Self {
            id,
            focus_at,
            blur_at,
        }
    }

    fn split_by_max_duration(&self) -> Vec<FocusRecord> {
        let mut focus_at = self.focus_at;
        let mut duration = self.duration();
        let mut ret = Vec::new();
        loop {
            if duration > DURATION_MAX {
                let blur_at = focus_at + DURATION_MAX;
                ret.push(FocusRecord::new(self.id, focus_at, blur_at));
                focus_at = blur_at;
                duration -= DURATION_MAX;
            } else {
                let blur_at = focus_at + duration;
                ret.push(FocusRecord::new(self.id, focus_at, blur_at));
                break;
            }
        }
        ret
    }

    fn split_by_not_across_day(&self) -> Vec<FocusRecord> {
        let mut focus_at = self.focus_at;
        let blur_at = self.blur_at;
        let mut ret = Vec::new();
        loop {
            if ms_as_d(focus_at) == ms_as_d(blur_at) {
                ret.push(FocusRecord::new(self.id, focus_at, blur_at));
                break;
            } else {
                let blur_at = start_of_next_d(focus_at);
                ret.push(FocusRecord::new(self.id, focus_at, blur_at));
                focus_at = blur_at;
            }
        }
        ret
    }
}

impl From<RecordByte> for FocusRecord {
    fn from(value: RecordByte) -> Self {
        FocusRecord::from_byte(value)
    }
}
