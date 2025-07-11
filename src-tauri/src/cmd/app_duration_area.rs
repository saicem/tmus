use crate::cmd::read_helper::{read_by_timestamp, timezone_convert};
use crate::util::time_util::date_str_from_days;
use serde::{Deserialize, Serialize};
use std::cmp::max;
use tmus_engine::models::FocusRecord;
use tmus_engine::util::{d_as_ms, ms_as_d, ms_as_m, Timestamp};

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
#[tracing::instrument]
pub async fn get_app_duration_area(
    app_id: usize,
    start_timestamp: Timestamp,
    end_timestamp: Timestamp,
    timezone_offset: Timestamp,
) -> AppDurationAreaModel {
    let result: Vec<FocusRecord> = timezone_convert(
        read_by_timestamp(start_timestamp, end_timestamp),
        timezone_offset,
    )
    .into_iter()
    .filter(|x| x.id == app_id)
    .collect();
    AppDurationAreaModel {
        app_id,
        date_area: compute_date_area(&result),
        day_area: compute_day_area(&result),
    }
}

fn compute_date_area(vec: &Vec<FocusRecord>) -> Vec<AppDurationAreaModelItem> {
    let Some(first) = vec.first() else {
        return vec![];
    };
    let Some(last) = vec.last() else {
        return vec![];
    };
    let first_day = ms_as_d(first.focus_at);
    let last_day = ms_as_d(last.blur_at);
    let size = (last_day - first_day + 1) as usize;
    let mut ret: Vec<AppDurationAreaModelItem> = Vec::with_capacity(size);
    for i in 0..size {
        ret.push(AppDurationAreaModelItem {
            index: date_str_from_days(first_day + i as i64),
            value: 0,
        });
    }

    for record in vec {
        let mut focus_at = record.focus_at;
        let blur_at = record.blur_at;
        while focus_at < blur_at {
            let focus_at_day = ms_as_d(focus_at);
            let blur_at_day = ms_as_d(blur_at);
            if focus_at_day == blur_at_day {
                ret[(focus_at_day - first_day) as usize].value += blur_at - focus_at;
                break;
            }
            let next_day_start = d_as_ms(focus_at_day + 1);
            ret[(focus_at_day - first_day) as usize].value += next_day_start - focus_at;
            focus_at = next_day_start;
        }
    }
    ret
}

/// Calculates the number of days an application is used each minute within the 24-hour period during the specified time range.
/// Minutes with usage are counted as one if used multiple times within the same day, and partial minutes are also counted.
fn compute_day_area(vec: &Vec<FocusRecord>) -> Vec<AppDurationAreaModelItem> {
    let size = 24 * 60;
    let mut ret = Vec::with_capacity(size);
    for i in 0..size {
        ret.push(AppDurationAreaModelItem {
            index: format!("{:02}:{:02}", i / 60, i % 60),
            value: 0,
        });
    }
    let mut pre_blur_at_minute = 0;
    for record in vec {
        let mut focus_at = record.focus_at;
        let blur_at = record.blur_at;
        while focus_at < blur_at {
            let focus_at_day = ms_as_d(focus_at);
            let blur_at_day = ms_as_d(blur_at);
            let base_day_minute = ms_as_m(d_as_ms(focus_at_day));
            let focus_at_minute = max(ms_as_m(focus_at), pre_blur_at_minute) - base_day_minute;
            pre_blur_at_minute = ms_as_m(blur_at) + 1;
            let blur_at_minute = pre_blur_at_minute - base_day_minute;
            pre_blur_at_minute = blur_at_minute;
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
            focus_at = d_as_ms(focus_at_day + 1);
        }
    }

    let mut current = 0;
    for item in &mut ret {
        current += item.value;
        item.value = current;
    }
    ret
}
