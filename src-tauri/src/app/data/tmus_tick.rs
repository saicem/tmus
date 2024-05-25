use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

const DAY_MILLISECONDS: u64 = Duration::days(1).num_milliseconds() as u64;

/// Tmus's atomic timing unit.
pub const TICK_PER_MILLIS: u64 = 10;

pub const DAY_TICK: u64 = DAY_MILLISECONDS / TICK_PER_MILLIS;

pub fn tick_to_millis(tick: u64) -> u64 {
    tick * TICK_PER_MILLIS
}

pub fn millis_to_tick(millis: u64) -> u64 {
    millis / TICK_PER_MILLIS
}

#[derive(Clone)]
pub struct TmusTick {
    pub tick: u64,
}

impl TmusTick {
    pub fn from_date_time(date_time: DateTime<Utc>) -> TmusTick {
        TmusTick {
            tick: date_time.timestamp_millis() as u64 / TICK_PER_MILLIS,
        }
    }

    pub fn now() -> TmusTick {
        Self::from_date_time(Utc::now())
    }

    pub fn day(&self) -> u64 {
        self.tick / DAY_TICK
    }

    pub fn tick_of_day(&self) -> u64 {
        self.tick % DAY_TICK
    }
}
