use crate::cmd::read_helper::{read_by_timestamp, timezone_convert};
use crate::engine::models::millisecond::Millisecond;
use crate::engine::models::AppId;
use crate::engine::FocusRecord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdDuration {
    app_id: AppId,
    duration: Millisecond,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateDuration {
    date: Millisecond,
    duration: Millisecond,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdDateDuration {
    app_id: AppId,
    date: Millisecond,
    duration: Millisecond,
}

#[tauri::command]
pub fn get_duration_by_id(
    start_timestamp: Millisecond,
    end_timestamp: Millisecond,
) -> Vec<IdDuration> {
    compute_duration_group_by_id(read_by_timestamp(start_timestamp, end_timestamp))
        .into_iter()
        .map(|(id, duration)| IdDuration {
            app_id: id,
            duration,
        })
        .collect()
}

#[tauri::command]
pub fn get_duration_by_date(
    start_timestamp: Millisecond,
    end_timestamp: Millisecond,
    timezone_offset: Millisecond,
) -> Vec<DateDuration> {
    compute_duration_group_by_date(timezone_convert(
        read_by_timestamp(start_timestamp, end_timestamp),
        timezone_offset,
    ))
    .into_iter()
    .map(|(day, duration)| DateDuration {
        date: Millisecond::from_days(day) + timezone_offset,
        duration,
    })
    .collect()
}

#[tauri::command]
pub fn get_duration_by_date_id(
    start_timestamp: Millisecond,
    end_timestamp: Millisecond,
    timezone_offset: Millisecond,
) -> Vec<IdDateDuration> {
    compute_duration_group_by_date_id(timezone_convert(
        read_by_timestamp(start_timestamp, end_timestamp),
        timezone_offset,
    ))
    .into_iter()
    .flat_map(|(day, app_durations)| {
        let date = Millisecond::from_days(day) + timezone_offset;
        app_durations
            .into_iter()
            .map(move |(app_id, duration)| IdDateDuration {
                date,
                app_id,
                duration,
            })
    })
    .collect()
}

pub fn compute_duration_group_by_id(records: Vec<FocusRecord>) -> HashMap<AppId, Millisecond> {
    records.into_iter().fold(HashMap::new(), |mut acc, record| {
        acc.entry(record.id)
            .and_modify(|x| *x += record.duration())
            .or_insert(record.duration());
        acc
    })
}

pub fn compute_duration_group_by_date(
    ordered_records: Vec<FocusRecord>,
) -> HashMap<i64, Millisecond> {
    let mut ret = HashMap::new();
    let mut add_duration = |day: i64, duration: Millisecond| {
        ret.entry(day)
            .and_modify(|x| *x += duration)
            .or_insert(duration);
    };
    for record in ordered_records.iter() {
        let focus_at = record.focus_at;
        let blur_at = record.blur_at;
        let focus_day = record.focus_at.as_days();
        let blur_day = record.blur_at.as_days();
        if focus_day == blur_day {
            add_duration(focus_day, blur_at - focus_at);
            continue;
        }
        let focus_day_add = Millisecond::from_days(focus_day + 1) - focus_at;
        add_duration(focus_day, focus_day_add);
        let blur_day_add = blur_at - Millisecond::from_days(blur_day);
        add_duration(blur_day, blur_day_add);
        for day in focus_day + 1..blur_day {
            add_duration(day, Millisecond::ONE_DAY);
        }
    }
    ret
}

pub fn compute_duration_group_by_date_id(
    ordered_records: Vec<FocusRecord>,
) -> HashMap<i64, HashMap<AppId, Millisecond>> {
    let mut ret: HashMap<i64, HashMap<AppId, Millisecond>> = HashMap::new();
    let mut add_duration = |day: i64, app_id: AppId, duration: Millisecond| {
        let duration_by_id_map = ret.entry(day).or_insert_with(HashMap::new);
        duration_by_id_map
            .entry(app_id)
            .and_modify(|x| *x += duration)
            .or_insert(duration);
    };
    for record in ordered_records.iter() {
        let focus_at = record.focus_at;
        let blur_at = record.blur_at;
        let focus_day = record.focus_at.as_days();
        let blur_day = record.blur_at.as_days();
        if focus_day == blur_day {
            add_duration(focus_day, record.id, blur_at - focus_at);
            continue;
        }
        let focus_day_add = Millisecond::from_days(focus_day + 1) - focus_at;
        add_duration(focus_day, record.id, focus_day_add);
        let blur_day_add = blur_at - Millisecond::from_days(blur_day);
        add_duration(blur_day, record.id, blur_day_add);
        for day in focus_day + 1..blur_day {
            add_duration(day, record.id, Millisecond::ONE_DAY);
        }
    }
    ret
}
