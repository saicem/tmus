use std::time::{SystemTime, UNIX_EPOCH};

type Unit = i64;
pub type Timestamp = Unit;
const SECOND: Unit = 1000;
const MINUTE: Unit = 60 * SECOND;
const HOUR: Unit = 60 * MINUTE;
const DAY: Unit = 24 * HOUR;

pub const fn s_as_ms(s: Unit) -> Unit {
    s * SECOND
}

pub const fn m_as_ms(m: Unit) -> Unit {
    m * MINUTE
}

pub const fn h_as_ms(h: Unit) -> Unit {
    h * HOUR
}

pub const fn d_as_ms(d: Unit) -> Unit {
    d * DAY
}

pub const fn ms_as_s(ms: Unit) -> Unit {
    ms / SECOND
}

pub const fn ms_as_m(ms: Unit) -> Unit {
    ms / MINUTE
}

pub const fn ms_as_h(ms: Unit) -> Unit {
    ms / HOUR
}

pub const fn ms_as_d(ms: Unit) -> Unit {
    ms / DAY
}

pub const fn start_of_d(ms: Unit) -> Unit {
    d_as_ms(ms_as_d(ms))
}

pub const fn start_of_next_d(ms: Unit) -> Unit {
    d_as_ms(ms_as_d(ms) + 1)
}

pub fn now_timestamp() -> Timestamp {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch");
    duration.as_millis() as Timestamp
}

pub fn now_day() -> Unit {
    now_timestamp() / DAY
}
