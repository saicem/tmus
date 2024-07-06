use serde::Serialize;

use super::millisecond::Millisecond;

const DURATION_MAX: Millisecond = Millisecond::from_secs(u16::MAX as i64);

pub type RecordByte = [u8; 8];

/// - app_id: 2^16, 8192 applications support.
/// - focus_at: 2^32, The time when the focus starts. About 136 year from unix epoch.
/// - duration: 2^16, Maximum time that can be represented is about 0.76 day,
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct FocusRecord {
    pub id: usize,
    pub focus_at: Millisecond,
    pub blur_at: Millisecond,
}

impl FocusRecord {
    pub fn new(id: usize, focus_at: Millisecond, blur_at: Millisecond) -> FocusRecord {
        FocusRecord {
            id,
            focus_at,
            blur_at,
        }
    }

    pub fn duration(&self) -> Millisecond {
        self.blur_at - self.focus_at
    }

    /// Convert to bytes. Use [`Self::split_record`] method ensure that the duration value is safe.
    pub fn unsafe_to_byte(&self) -> RecordByte {
        let mut ret = RecordByte::default();
        let id = self.id as u16;
        let focus_at = self.focus_at.as_secs() as u32;
        let duration = self.duration().as_secs() as u16;
        ret[..2].copy_from_slice(&id.to_le_bytes());
        ret[2..4].copy_from_slice(&duration.to_le_bytes());
        ret[4..].copy_from_slice(&focus_at.to_le_bytes());
        ret
    }

    fn from_byte(bytes: RecordByte) -> Self {
        let id = u16::from_le_bytes(bytes[..2].try_into().unwrap()) as usize;
        let focus_at =
            Millisecond::from_secs(u32::from_le_bytes(bytes[4..].try_into().unwrap()) as i64);
        let blur_at = focus_at
            + Millisecond::from_secs(u16::from_le_bytes(bytes[2..4].try_into().unwrap()) as i64);
        Self {
            id,
            focus_at,
            blur_at,
        }
    }

    /// Split record into multiple records. Ensure that the duration of each record is less than 0.76 day(The maximum time that can be represented).
    pub fn split_record(&self) -> Vec<FocusRecord> {
        let mut ret = Vec::<FocusRecord>::new();
        let mut focus_at = self.focus_at;
        let mut duration = self.blur_at - self.focus_at;
        while duration > DURATION_MAX {
            ret.push(FocusRecord::new(self.id, focus_at, focus_at + DURATION_MAX));
            focus_at += DURATION_MAX;
            duration -= DURATION_MAX;
        }
        ret.push(FocusRecord::new(self.id, focus_at, focus_at + duration));
        ret
    }
}

impl From<RecordByte> for FocusRecord {
    fn from(value: RecordByte) -> Self {
        FocusRecord::from_byte(value)
    }
}
