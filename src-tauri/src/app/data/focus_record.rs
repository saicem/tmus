use super::Tick;

/// The meaning of 64bit as follows
/// - app_id: 0..16
/// - focus_at: 16..40
/// - duration: 40..64
#[derive(Clone)]
pub struct FocusRecord {
    pub raw: u64,
}

impl FocusRecord {
    pub fn new(app_id: u64, focus_at: Tick, duration: Tick) -> Self {
        FocusRecord {
            raw: app_id | focus_at.0 << 16 | duration.0 << 40,
        }
    }

    pub fn app_id(&self) -> u64 {
        self.raw & 0xffff
    }

    pub fn focus_at(&self) -> Tick {
        Tick((self.raw >> 16) & 0xffffff)
    }

    pub fn duration(&self) -> Tick {
        Tick((self.raw >> 40) & 0xffffff)
    }

    pub fn blur_at(&self) -> Tick {
        self.focus_at() + self.duration()
    }
}
