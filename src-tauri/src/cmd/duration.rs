use crate::cmd::read_helper::read_by_timestamp;
use crate::engine::models::AppId;
use crate::engine::util::Timestamp;
use crate::engine::FocusRecord;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

/// MergeOperation defines the type of aggregation operation to perform.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeOperation {
    /// Sums up all durations within intervals
    Sum,
    /// Counts each interval that contains any activity
    Count,
    /// Counts only intervals where activity completely fills the interval
    CountIfCompletelyContain,
}

/// complex_query processes focus records with customizable filters and aggregation operations.
///
/// # Arguments
/// * `start_timestamp` - Start time for filtering records (inclusive)
/// * `end_timestamp` - End time for filtering records (inclusive)
/// * `is_merge_apps` - Whether to merge data across all apps into one result set
/// * `app_ids` - Optional list of app IDs to filter by
/// * `interval` - Time interval in milliseconds to split records into
/// * `operation` - The aggregation operation to apply
///
/// # Returns
/// A vector of tuples containing:
/// - App ID (or 0 if merged)
/// - Interval index
/// - Computed value based on operation
#[tauri::command]
pub async fn complex_query(
    start_timestamp: Timestamp,
    end_timestamp: Timestamp,
    is_merge_apps: bool,
    app_ids: Option<HashSet<AppId>>,
    interval: Timestamp,
    operation: MergeOperation,
) -> Vec<(usize, i64, i64)> {
    let mut raw = read_by_timestamp(start_timestamp, end_timestamp);

    if let Some(app_ids) = app_ids {
        raw = raw
            .into_iter()
            .filter(|item| app_ids.contains(&item.id))
            .collect();
    }

    if is_merge_apps {
        raw.iter_mut().for_each(|item| item.id = 0)
    }

    let data_map = aggregate_by_interval(&raw, start_timestamp, interval, operation);
    data_map
        .into_iter()
        .map(|((id, index), value)| (id, index, value))
        .collect()
}

fn aggregate_by_interval(
    ordered_data: &Vec<FocusRecord>,
    start_timestamp: Timestamp,
    interval: Timestamp,
    operation: MergeOperation,
) -> HashMap<(usize, i64), i64> {
    if ordered_data.is_empty() {
        return HashMap::new();
    }

    ordered_data
        .iter()
        .flat_map(|x| SplitRecord::new(x.id, x.focus_at, x.blur_at, start_timestamp, interval))
        .fold(HashMap::new(), |mut acc, (app_id, index, duration)| {
            let val = acc.entry((app_id, index)).or_insert(0);
            *val = match operation {
                MergeOperation::Sum => *val + duration,
                MergeOperation::Count => 1,
                MergeOperation::CountIfCompletelyContain => *val + (duration == interval) as i64,
            };
            acc
        })
}

struct SplitRecord {
    app_id: usize,
    focus_at: i64,
    blur_at: i64,
    cursor: i64,
    interval: i64,
}

impl SplitRecord {
    fn new(
        app_id: usize,
        focus_at: i64,
        blur_at: i64,
        start_timestamp: i64,
        interval: i64,
    ) -> Self {
        Self {
            app_id,
            focus_at,
            blur_at,
            cursor: focus_at - (focus_at - start_timestamp) % interval,
            interval,
        }
    }
}

impl Iterator for SplitRecord {
    type Item = (usize, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.blur_at {
            return None;
        }

        let next_cursor = self.cursor + self.interval;
        let ret = (
            self.app_id,
            self.cursor,
            min(self.blur_at, next_cursor) - max(self.focus_at, self.cursor),
        );
        self.cursor = next_cursor;
        Some(ret)
    }
}
