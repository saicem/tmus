use crate::cmd::read_helper::read_by_timestamp;
use crate::error::TmusError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::Not;
use tmus_engine::models::{AppId, FocusRecord};
use tmus_engine::util::Timestamp;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdDuration {
    pub app_id: usize,
    pub duration: i64,
}

/// Aggregate total duration per app within [start, end). Shared by the Tauri
/// command and the CLI.
pub fn duration_by_id(start_timestamp: Timestamp, end_timestamp: Timestamp) -> Vec<IdDuration> {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DurationByIdRequest {
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
}

#[tauri::command]
#[tracing::instrument(skip(request))]
pub fn get_duration_by_id(request: DurationByIdRequest) -> Vec<IdDuration> {
    duration_by_id(request.start_timestamp, request.end_timestamp)
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DurationStatisticRequest {
    /// The start time (inclusive) for filtering focus records.
    pub start_timestamp: Timestamp,
    /// The end time (inclusive) for filtering focus records.
    pub end_timestamp: Timestamp,
    /// Whether to merge data across all applications into a single result set.
    pub merge_apps: bool,
    /// Optional list of application IDs to filter by.
    pub app_ids: Option<HashSet<AppId>>,
    /// The time interval (in milliseconds) used to split records for aggregation.
    pub granularity: Timestamp,
    /// Optional cycle length in units of `granularity` for repeating cycles.
    pub cycle: Option<i64>,
}

/// Query duration statistics with customizable filters and aggregation.
#[tauri::command]
#[tracing::instrument(skip(request))]
pub async fn query_duration_statistic(
    request: DurationStatisticRequest,
) -> Result<Vec<DurationStat>, TmusError> {
    if request.start_timestamp >= request.end_timestamp {
        return Err(TmusError::new(
            "invalid_time_range",
            "start_timestamp must be less than end_timestamp",
        ));
    }

    let mut raw_data = read_by_timestamp(request.start_timestamp, request.end_timestamp);

    if let Some(selected_app_ids) = request.app_ids {
        raw_data.retain(|record| selected_app_ids.contains(&record.id));
    }

    let aggregated_data = aggregate_by_interval(
        &raw_data,
        request.start_timestamp,
        request.granularity,
        request.merge_apps,
        request.cycle,
    );

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
            (interval_start - base_timestamp) % cycle_duration
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
