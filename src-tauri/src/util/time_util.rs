use crate::engine::models::millisecond::MILLISECOND_PER_DAY;
use chrono::{DateTime, Utc};

pub fn date_time_from_days(days: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(days * MILLISECOND_PER_DAY).unwrap()
}

pub fn date_str_from_days(days: i64) -> String {
    date_time_from_days(days).format("%Y-%m-%d").to_string()
}
