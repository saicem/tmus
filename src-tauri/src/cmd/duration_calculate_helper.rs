use crate::engine::data::AppId;
use crate::engine::data::Millisecond;
use crate::engine::FocusRecord;
use std::collections::{BTreeMap, HashMap};

pub fn group_by_id(records: Vec<FocusRecord>) -> HashMap<AppId, Millisecond> {
    let mut ret = HashMap::new();
    for record in records.iter() {
        *ret.entry(record.id).or_insert(Millisecond::ZERO) += record.duration();
    }
    ret
}

pub fn group_by_day(
    records: Vec<FocusRecord>,
    time_zone_offset: Millisecond,
) -> HashMap<i64, Millisecond> {
    let mut ret = HashMap::new();
    for mut record in records.into_iter() {
        record.focus_at += time_zone_offset;
        record.blur_at += time_zone_offset;
        let start_day = record.focus_at.as_days();
        let end_day = record.blur_at.as_days();
        if start_day == end_day {
            *ret.entry(start_day).or_insert(Millisecond::ZERO) += record.duration();
        } else {
            *ret.entry(start_day).or_insert(Millisecond::ZERO) +=
                Millisecond::ONE_DAY - record.focus_at.get_day_offset();
            *ret.entry(end_day).or_insert(Millisecond::ZERO) += record.blur_at.get_day_offset();
        }
    }
    ret
}

pub fn group_by_day_id(
    records: Vec<FocusRecord>,
    time_zone_offset: Millisecond,
) -> HashMap<i64, BTreeMap<AppId, Millisecond>> {
    let mut ret: HashMap<i64, BTreeMap<AppId, Millisecond>> = HashMap::new();
    for mut record in records.into_iter() {
        record.focus_at += time_zone_offset;
        record.blur_at += time_zone_offset;
        let start_day = record.focus_at.as_days();
        let end_day = record.blur_at.as_days();
        if start_day == end_day {
            *ret.entry(start_day)
                .or_insert(BTreeMap::new())
                .entry(record.id)
                .or_insert(Millisecond::ZERO) += record.duration();
        } else {
            *ret.entry(start_day)
                .or_insert(BTreeMap::new())
                .entry(record.id)
                .or_insert(Millisecond::ZERO) +=
                Millisecond::ONE_DAY - record.focus_at.get_day_offset();
            *ret.entry(end_day)
                .or_insert(BTreeMap::new())
                .entry(record.id)
                .or_insert(Millisecond::ZERO) += record.blur_at.get_day_offset();
        }
    }
    ret
}
