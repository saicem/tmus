use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Millisecond(i64);

impl Millisecond {
    pub fn now() -> Millisecond {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch");
        Millisecond(duration.as_secs() as i64 * 1000 + duration.as_millis() as i64)
    }

    pub fn as_days(&self) -> i64 {
        self.0 / (1000 * 60 * 60 * 24)
    }

    pub const fn from_days(days: i64) -> Millisecond {
        Millisecond(days * 1000 * 60 * 60 * 24)
    }

    pub fn as_secs(&self) -> i64 {
        self.0 / 1000
    }

    pub const fn from_secs(secs: i64) -> Millisecond {
        Millisecond(secs * 1000)
    }

    pub const fn from_millis(millis: i64) -> Millisecond {
        Millisecond(millis)
    }

    pub const ZERO: Millisecond = Millisecond(0);
}

impl Display for Millisecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let second = (self.0 / 1000) % (60 * 60 * 24);
        let minute = (second / 60) % 60;
        let hour = second / 60 / 60;
        let second = second % 60;
        f.write_str(&format!("{}:{}:{}", hour, minute, second))
    }
}

impl Serialize for Millisecond {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'de> Deserialize<'de> for Millisecond {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        i64::deserialize(deserializer).map(Millisecond)
    }
}

impl Add for Millisecond {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Millisecond(self.0 + other.0)
    }
}

impl AddAssign for Millisecond {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Millisecond {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Millisecond(self.0 - other.0)
    }
}

impl SubAssign for Millisecond {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_millisecond_compute() {
        let mut ms1 = Millisecond(1000);
        let ms2 = Millisecond(500);

        ms1 += ms2;
        assert_eq!(ms1, Millisecond(1500));

        ms1 -= ms2;
        assert_eq!(ms1, Millisecond(1000));

        assert!(ms1 > ms2);
        assert!(ms2 < ms1);
    }

    #[test]
    fn test_millisecond_serde() {
        let ms = Millisecond(1000);
        let json = serde_json::to_string(&ms).unwrap();
        assert_eq!(json, "1000");
        let ms2: Millisecond = serde_json::from_str(&json).unwrap();
        assert_eq!(ms, ms2);
    }
}
