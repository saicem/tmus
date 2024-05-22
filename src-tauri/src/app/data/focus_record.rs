/// The meaning of 64bit as follows
/// - app_id: 0..16
/// - focus_at: 16..40
/// - duration: 40..64
#[derive(Clone)]
pub struct FocusRecord {
    pub(crate) data: u64,
}

impl FocusRecord {
    pub fn new(app_id: u64, focus_at: u64, duration: u64) -> Self {
        FocusRecord {
            data: app_id | focus_at << 16 | duration << 40,
        }
    }

    pub fn app_id(&self) -> u64 {
        self.data & 0xffff
    }

    pub fn focus_at(&self) -> u64 {
        (self.data >> 16) & 0xffffff
    }

    pub fn duration(&self) -> u64 {
        (self.data >> 40) & 0xffffff
    }

    pub fn blur_at(&self) -> u64 {
        self.focus_at() + self.duration()
    }
}
