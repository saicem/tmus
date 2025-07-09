use chrono::{DateTime, Utc};
use tmus_engine::util::d_as_ms;

pub fn date_time_from_days(days: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(d_as_ms(days)).unwrap()
}

pub fn date_str_from_days(days: i64) -> String {
    date_time_from_days(days).format("%Y-%m-%d").to_string()
}
