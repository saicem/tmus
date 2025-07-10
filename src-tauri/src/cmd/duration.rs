use crate::cmd::read_helper::read_by_timestamp;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::Not;
use tmus_engine::models::AppId;
use tmus_engine::util::Timestamp;
use tmus_engine::FocusRecord;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdDuration {
    app_id: usize,
    duration: i64,
}

#[tauri::command]
pub fn get_duration_by_id(start_timestamp: Timestamp, end_timestamp: Timestamp) -> Vec<IdDuration> {
    read_by_timestamp(start_timestamp, end_timestamp)
        .into_iter()
        .fold(HashMap::new(), |mut acc, record| {
            acc.entry(record.id)
                .and_modify(|x| *x += record.duration())
                .or_insert(record.duration());
            acc
        })
        .into_iter()
        .map(|(id, duration)| IdDuration {
            app_id: id,
            duration,
        })
        .collect()
}

/// Represents a duration statistic for a specific application within a time interval
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DurationStat {
    /// Application ID (None if apps are merged)
    app_id: Option<usize>,
    /// Start timestamp of the interval (in milliseconds)
    interval_start: i64,
    /// Total focused duration within this interval (in milliseconds)
    duration: i64,
}

/// Query duration statistics with customizable filters and aggregation.
///
/// # Arguments
///
/// * `start_timestamp` - The start time (inclusive) for filtering focus records.
/// * `end_timestamp` - The end time (inclusive) for filtering focus records.
/// * `merge_apps` - Whether to merge data across all applications into a single result set.  
///   Set this to `true` if application-specific details are not required.
/// * `app_ids` - An optional list of application IDs to filter by.  
///   If only specific applications are of interest, provide their IDs here.
/// * `granularity` - The time interval (in milliseconds) used to split records for aggregation.  
///   For example:
///   - Use `86400000` to aggregate by day.
///   - Use `3600000` to aggregate by hour.
/// * `cycle` - An optional cycle length in units of `granularity`.  
///   This allows grouping intervals into repeating cycles.  
///   For example:
///   - To analyze how much time is spent each hour within a day, use `granularity=3600000` and `cycle=24`.
///   - To analyze how much time is spent each day within a week, use `granularity=86400000` and `cycle=7`.
#[tauri::command]
pub async fn query_duration_statistic(
    start_timestamp: Timestamp,
    end_timestamp: Timestamp,
    merge_apps: bool,
    app_ids: Option<HashSet<AppId>>,
    granularity: Timestamp,
    cycle: Option<i64>,
) -> Result<Vec<DurationStat>, &'static str> {
    if start_timestamp >= end_timestamp {
        return Err("start_timestamp must be greater than end_timestamp");
    }

    let mut raw_data = read_by_timestamp(start_timestamp, end_timestamp);

    if let Some(selected_app_ids) = app_ids {
        raw_data = raw_data
            .into_iter()
            .filter(|record| selected_app_ids.contains(&record.id))
            .collect();
    }

    let aggregated_data =
        aggregate_by_interval(&raw_data, start_timestamp, granularity, merge_apps, cycle);

    Ok(aggregated_data
        .into_iter()
        .map(|((app_id, interval_start), duration)| DurationStat {
            app_id,
            interval_start,
            duration,
        })
        .collect())
}

fn aggregate_by_interval(
    focus_records: &Vec<FocusRecord>,
    base_timestamp: Timestamp,
    interval_millis: Timestamp,
    merge_apps: bool,
    cycle: Option<i64>,
) -> HashMap<(Option<usize>, i64), i64> {
    if focus_records.is_empty() {
        return HashMap::new();
    }
    let cycle_duration = cycle.map(|cycle| cycle * interval_millis);
    let mut aggregated_data = HashMap::new();
    let mut add_data = |app_id: usize, interval_start: i64, duration: i64| {
        let id = merge_apps.not().then_some(app_id);
        let interval_start = cycle_duration.map_or(interval_start, |cycle_duration| {
            (interval_start - interval_millis) % cycle_duration
        });
        aggregated_data
            .entry((id, interval_start))
            .and_modify(|value| *value += duration)
            .or_insert(duration);
    };
    for record in focus_records {
        let mut interval_start =
            record.focus_at - (record.focus_at - base_timestamp) % interval_millis;
        let mut interval_end = interval_start + interval_millis;
        if record.blur_at < interval_end {
            add_data(record.id, interval_start, record.blur_at - record.focus_at);
        } else {
            add_data(record.id, interval_start, interval_end - record.focus_at);
            (interval_start, interval_end) = (interval_end, interval_end + interval_millis);
            while interval_end < record.blur_at {
                add_data(record.id, interval_start, interval_millis);
                (interval_start, interval_end) = (interval_end, interval_end + interval_millis);
            }
            add_data(record.id, interval_start, record.blur_at - interval_start);
        }
    }
    aggregated_data
}
