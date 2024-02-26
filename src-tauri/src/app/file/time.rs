use chrono::{DateTime, Duration, Utc};

pub fn today() -> u64 {
    Utc::now().timestamp_millis() as u64 / DAY_MILLI_SEC
}

pub fn custom_now() -> (u64, u64) {
    let now = Utc::now().timestamp_millis();
    let day = now as u64 / DAY_MILLI_SEC;
    let time = now as u64 % DAY_MILLI_SEC / 10;
    (day, time)
}

pub fn custom_datetime(date_time: DateTime<Utc>) -> (u64, u64) {
    let millis = date_time.timestamp_millis();
    let day = millis as u64 / DAY_MILLI_SEC;
    let time = millis as u64 % DAY_MILLI_SEC / 10;
    (day, time)
}

const DAY_MILLI_SEC: u64 = Duration::days(1).num_milliseconds() as u64;
pub const DAY_CENTI_SEC: u64 = DAY_MILLI_SEC / 10;
