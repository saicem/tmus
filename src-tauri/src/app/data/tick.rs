use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

const TICK_PER_MILLIS: u64 = 10;
const DAY_TICK: u64 = Duration::days(1).num_milliseconds() as u64 / TICK_PER_MILLIS;

#[derive(Debug, Clone, Copy)]
pub struct Tick(pub u64);

impl Tick {
    pub fn now() -> Tick {
        Tick::from_utc(&Utc::now())
    }

    pub fn from_days(days: u64) -> Tick {
        Tick(days * DAY_TICK)
    }

    pub fn from_utc(date_time: &DateTime<Utc>) -> Tick {
        Tick::from_millis(date_time.timestamp_millis() as u64)
    }

    /// Risk of losing accuracy. The `Tick`'s unit is 10 milliseconds.
    pub fn from_millis(millis: u64) -> Tick {
        Tick(millis / TICK_PER_MILLIS)
    }

    pub fn to_millis(&self) -> u64 {
        self.0 * TICK_PER_MILLIS
    }

    pub fn day_tick(&self) -> Tick {
        Tick(self.0 % DAY_TICK)
    }

    pub fn day(&self) -> u64 {
        self.0 / DAY_TICK
    }
}

impl Add for Tick {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Tick {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Ord for Tick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Tick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for Tick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Tick {}

impl Sum for Tick {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Tick::from_millis(0), |acc, x| acc + x)
    }
}

impl AddAssign for Tick {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
