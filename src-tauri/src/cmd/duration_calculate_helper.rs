use crate::engine::models::AppId;
use crate::engine::models::Millisecond;
use crate::engine::FocusRecord;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DurationByIdItem {
    app_id: AppId,
    duration: Millisecond,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DurationByDayItem {
    day_epoch: i64,
    duration: Millisecond,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DurationByDayIdItem {
    app_id: AppId,
    day_epoch: i64,
    duration: Millisecond,
}

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
            debug_assert!(end_day - start_day == 1, "end_day - start_day must be 1");
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
