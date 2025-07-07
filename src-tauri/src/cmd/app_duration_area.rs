use crate::cmd::read_helper::read_by_timestamp;
use crate::engine::data::Millisecond;
use crate::engine::FocusRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDurationAreaModel {
    app_id: usize,
    date_area: Vec<AppDurationAreaModelItem>,
    day_area: Vec<AppDurationAreaModelItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDurationAreaModelItem {
    index: String,
    value: i64,
}

#[tauri::command]
pub async fn get_app_duration_area(
    app_id: usize,
    start_millis: Millisecond,
    end_millis: Millisecond,
    time_zone_offset: Millisecond,
) -> AppDurationAreaModel {
    let result: Vec<FocusRecord> = read_by_timestamp(start_millis, end_millis)
        .into_iter()
        .filter(|x| x.id == app_id)
        .collect();
    AppDurationAreaModel {
        app_id,
        date_area: compute_date_area(&result, time_zone_offset),
        day_area: compute_day_area(&result, time_zone_offset),
    }
}

fn compute_date_area(
    vec: &Vec<FocusRecord>,
    time_zone_offset: Millisecond,
) -> Vec<AppDurationAreaModelItem> {
    static UNIX_EPOCH: DateTime<Utc> = DateTime::from_timestamp(0, 0).unwrap();
    let Some(first) = vec.first() else {
        return vec![];
    };
    let Some(last) = vec.last() else {
        return vec![];
    };
    let first_day = (first.focus_at + time_zone_offset).as_days();
    let last_day = (last.blur_at + time_zone_offset).as_days();
    let size = (last_day - first_day + 1) as usize;
    let mut ret: Vec<AppDurationAreaModelItem> = Vec::with_capacity(size);
    for i in 0..size {
        ret.push(AppDurationAreaModelItem {
            index: UNIX_EPOCH
                .add(chrono::Duration::days(first_day + i as i64))
                .format("%Y-%m-%d")
                .to_string(),
            value: 0,
        });
    }

    for record in vec {
        let mut focus_at = record.focus_at + time_zone_offset;
        let blur_at = record.blur_at + time_zone_offset;
        while focus_at < blur_at {
            let focus_at_day = focus_at.as_days();
            let blur_at_day = blur_at.as_days();
            if focus_at_day == blur_at_day {
                ret[(focus_at_day - first_day) as usize].value += (blur_at - focus_at).as_millis();
                break;
            }
            let next_day_start = Millisecond::from_days(focus_at_day + 1);
            ret[(focus_at_day - first_day) as usize].value +=
                (next_day_start - focus_at).as_millis();
            focus_at = next_day_start;
        }
    }
    ret
}

fn compute_day_area(
    vec: &Vec<FocusRecord>,
    time_zone_offset: Millisecond,
) -> Vec<AppDurationAreaModelItem> {
    let size = 24 * 60;
    let mut ret = Vec::with_capacity(size);
    for i in 0..size {
        ret.push(AppDurationAreaModelItem {
            index: format!("{:02}:{:02}", i / 60, i % 60),
            value: 0,
        });
    }
    for record in vec {
        let mut focus_at = record.focus_at + time_zone_offset;
        let blur_at = record.blur_at + time_zone_offset;
        while focus_at < blur_at {
            let focus_at_day = focus_at.as_days();
            let blur_at_day = blur_at.as_days();
            let base_day_minute = Millisecond::from_days(focus_at_day).as_minute();
            let focus_at_minute = focus_at.as_minute() + 1 - base_day_minute;
            let blur_at_minute = blur_at.as_minute() + 1 - base_day_minute;
            if focus_at_day == blur_at_day {
                if blur_at_minute > focus_at_minute && blur_at_minute < size as i64 {
                    ret[focus_at_minute as usize].value += 1;
                    ret[blur_at_minute as usize].value -= 1;
                }
                break;
            }
            if focus_at_minute < size as i64 {
                ret[focus_at_minute as usize].value += 1;
            }
            focus_at = Millisecond::from_days(focus_at_day + 1);
        }
    }

    let mut current = 0;
    for item in &mut ret {
        current += item.value;
        item.value = current;
    }
    ret
}
