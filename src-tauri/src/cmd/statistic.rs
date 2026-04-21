use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};
use tmus_engine::{
    models::AppId,
    util::{d_as_ms, ms_as_d, Timestamp},
};

use crate::{
    cmd::{
        app_detail::{get_all_app_detail, FileDetail},
        read_helper,
    },
    state::category::{
        get_app_category_map, get_category_detail_map, get_category_self_and_descendants_map,
        get_category_tree, CategoryId, CategoryNode, CategorySimple, UNCATEGORIZED_CATEGORY_ID,
    },
};

type Value = i64;
static UNCATEGORIZED: &str = "uncategorized";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TimeSpan {
    Day,
    Week,
}

impl TimeSpan {
    pub fn to_ms(&self) -> Timestamp {
        match self {
            Self::Day => d_as_ms(1),
            Self::Week => d_as_ms(7),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStatisticDetail {
    pub value: Value,
    pub app: FileDetail,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryStatisticDetail {
    pub value: Value,
    pub category: CategorySimple,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDurationRequest {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub category_id: Option<CategoryId>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDurationResponse {
    pub detail: Vec<AppStatisticDetail>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDayCountRequest {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub category_id: Option<CategoryId>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDayCountResponse {
    pub detail: Vec<AppStatisticDetail>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDurationRequest {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub category_ids: Vec<CategoryId>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDurationResponse {
    pub detail: Vec<CategoryStatisticDetail>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDayCountRequest {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub category_ids: Vec<CategoryId>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDayCountResponse {
    pub detail: Vec<CategoryStatisticDetail>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RhythmGroup {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub category_id: CategoryId,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RhythmRequest {
    pub groups: Vec<RhythmGroup>,
    pub span: TimeSpan,
    pub granularity: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RhythmDataResponse {
    pub values: Vec<Vec<Timestamp>>,
}

#[tauri::command]
pub fn get_base_time() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let days = since_epoch.as_secs() / (24 * 60 * 60);
    days * 24 * 60 * 60
}

#[tauri::command(async)]
pub async fn get_app_total_duration(
    request: AppDurationRequest,
) -> Result<AppDurationResponse, String> {
    let records = read_helper::read_by_timestamp(request.start_time, request.end_time);
    let mut map = HashMap::new();
    for record in records {
        let app_id = record.id;
        let duration = record.duration();
        *map.entry(app_id).or_insert(0) += duration;
    }
    if let Some(category_id) = request.category_id {
        let valid_app_ids = get_category_and_descendants_app_ids(&category_id)?;
        map.retain(|app_id, _| valid_app_ids.contains(app_id));
    }
    Ok(AppDurationResponse {
        detail: match_app_detail(map.into_iter().map(|(app_id, value)| (app_id, value))).await,
    })
}

#[tauri::command(async)]
pub async fn get_app_usage_days(
    request: AppDurationRequest,
) -> Result<AppDayCountResponse, String> {
    let records = read_helper::read_by_timestamp(request.start_time, request.end_time);
    let timezone_offset = infer_timezone_offset(request.start_time);
    let mut map: HashMap<AppId, HashSet<i64>> = HashMap::new();
    for record in records {
        let start_day = ms_as_d(record.focus_at + timezone_offset);
        let end_day = ms_as_d(record.blur_at + timezone_offset);
        let entry = map.entry(record.id).or_insert_with(HashSet::new);
        entry.insert(start_day);
        if end_day != start_day {
            entry.insert(end_day);
        }
    }
    if let Some(category_id) = request.category_id {
        let valid_app_ids = get_category_and_descendants_app_ids(&category_id)?;
        map.retain(|app_id, _| valid_app_ids.contains(app_id));
    }
    Ok(AppDayCountResponse {
        detail: match_app_detail(
            map.into_iter()
                .map(|(app_id, set)| (app_id, set.len() as i64)),
        )
        .await,
    })
}

#[tauri::command]
pub fn get_category_total_duration(
    request: CategoryDurationRequest,
) -> Result<CategoryDurationResponse, String> {
    if request.category_ids.is_empty() {
        return Err("category_ids is empty".to_string());
    }

    let app_category_map = get_app_category_map();
    let mut categorized_duration_map: HashMap<CategoryId, i64> = HashMap::new();
    let mut uncategorized_duration = 0;
    let records = read_helper::read_by_timestamp(request.start_time, request.end_time);
    for record in records {
        let category_id = app_category_map.get(&record.id);
        if let Some(category_id) = category_id {
            *categorized_duration_map.entry(*category_id).or_insert(0) += record.duration();
        } else {
            uncategorized_duration += record.duration();
        }
    }

    let category_self_and_descendants_map = get_category_self_and_descendants_map();
    let mut category_with_descendants_duration_map = HashMap::new();
    let mut include_uncategorized_category = false;
    for category_id in &request.category_ids {
        if let Some(self_descendants_category_ids) =
            category_self_and_descendants_map.get(&category_id)
        {
            let value = self_descendants_category_ids
                .iter()
                .map(|id| categorized_duration_map.get(id).unwrap_or(&0))
                .sum::<i64>();
            category_with_descendants_duration_map.insert(category_id, value);
        } else {
            include_uncategorized_category = true;
        }
    }

    let mut detail = match_category_detail(
        categorized_duration_map
            .into_iter()
            .map(|(category_id, value)| (category_id, value)),
    );

    if uncategorized_duration > 0 && include_uncategorized_category {
        detail.push(CategoryStatisticDetail {
            category: CategorySimple {
                id: UNCATEGORIZED_CATEGORY_ID,
                parent_id: UNCATEGORIZED_CATEGORY_ID,
                name: UNCATEGORIZED.into(),
            },
            value: uncategorized_duration,
        });
    }

    Ok(CategoryDurationResponse { detail })
}

#[tauri::command]
pub fn get_category_usage_days(
    request: CategoryDayCountRequest,
) -> Result<CategoryDayCountResponse, String> {
    if request.category_ids.is_empty() {
        return Err("category_ids is empty".to_string());
    }

    let app_category_map = get_app_category_map();
    let timezone_offset = infer_timezone_offset(request.start_time);
    let mut uncategorized_days = HashSet::new();
    let mut categorized_days_map: HashMap<CategoryId, HashSet<i64>> = HashMap::new();
    let records = read_helper::read_by_timestamp(request.start_time, request.end_time);
    for record in records {
        let category_id = app_category_map.get(&record.id);
        let start_day = ms_as_d(record.focus_at + timezone_offset);
        let end_day = ms_as_d(record.blur_at + timezone_offset);
        if let Some(category_id) = category_id {
            let entry = categorized_days_map
                .entry(*category_id)
                .or_insert_with(HashSet::new);
            entry.insert(start_day);
            if end_day != start_day {
                entry.insert(end_day);
            }
        } else {
            uncategorized_days.insert(start_day);
            if end_day != start_day {
                uncategorized_days.insert(end_day);
            }
        }
    }

    let category_and_descendants_map = get_category_self_and_descendants_map();
    let mut category_with_descendants_days_count_map = HashMap::new();
    let mut include_uncategorized_category = false;
    for category_id in request.category_ids {
        if let Some(self_descendants_category_ids) = category_and_descendants_map.get(&category_id)
        {
            let mut set: HashSet<i64> = HashSet::new();
            self_descendants_category_ids
                .iter()
                .filter_map(|id| categorized_days_map.get(id))
                .for_each(|category_set| set.extend(category_set));
            category_with_descendants_days_count_map.insert(category_id, set.len());
        } else {
            include_uncategorized_category = true;
        }
    }

    let mut detail = match_category_detail(
        category_with_descendants_days_count_map
            .into_iter()
            .map(|(category_id, value)| (category_id, value as i64)),
    );

    if uncategorized_days.len() > 0 && include_uncategorized_category {
        detail.push(CategoryStatisticDetail {
            category: CategorySimple {
                id: UNCATEGORIZED_CATEGORY_ID,
                parent_id: UNCATEGORIZED_CATEGORY_ID,
                name: UNCATEGORIZED.into(),
            },
            value: uncategorized_days.len() as i64,
        });
    }
    Ok(CategoryDayCountResponse { detail })
}

#[tauri::command]
pub fn get_category_usage_rhythm(request: RhythmRequest) -> Result<RhythmDataResponse, String> {
    // Valid check
    let time_span = request.span.to_ms();
    let granularity = request.granularity;
    if time_span % granularity != 0 {
        return Err("span must be a multiple of granularity".to_string());
    }
    let multiple = time_span / granularity;
    if multiple > 24 * 60 {
        return Err("granularity is too small".to_string());
    }
    if request.groups.is_empty() {
        return Err("groups is empty".to_string());
    }
    for group in &request.groups {
        if group.start_time >= group.end_time {
            return Err("start_time must be before end_time".to_string());
        }
        if (group.end_time - group.start_time) % time_span != 0 {
            return Err("end_time must be a multiple of span".to_string());
        }
    }

    // Compute result
    let timezone_offset = infer_timezone_offset(request.groups[0].start_time);
    let result = request
        .groups
        .into_iter()
        .map(
            |RhythmGroup {
                 start_time,
                 end_time,
                 category_id,
             }| {
                let valid_app_ids = get_category_and_descendants_app_ids(&category_id);
                if valid_app_ids.is_err() {
                    tracing::error!(
                        "get_category_and_descendants_app_ids error: {:?}",
                        category_id
                    );
                    return Vec::new();
                }
                let valid_app_ids = valid_app_ids.unwrap();

                let records = read_helper::read_by_timestamp(start_time, end_time);
                // This vec use for store less than granularity duration
                let mut partial_vec: Vec<Timestamp> = Vec::with_capacity(multiple as usize);
                // This vec use for store fully occupied granularity duration in adjacent difference vec
                let mut full_adj_dif_vec: Vec<i64> = Vec::with_capacity(multiple as usize);

                for record in records {
                    if !valid_app_ids.contains(&record.id) {
                        continue;
                    }
                    let start_tz = record.focus_at + timezone_offset;
                    let end_tz = record.blur_at + timezone_offset;
                    let start_in_span = start_tz % time_span;
                    let end_in_span = end_tz % time_span;
                    let cycle = (end_tz - start_tz) / time_span;
                    let start_in_granularity = start_in_span % granularity;
                    let end_in_granularity = end_in_span % granularity;
                    let start_index = start_in_granularity / granularity;
                    let end_index = end_in_granularity / granularity;

                    if start_index == end_index {
                        partial_vec[start_index as usize] +=
                            end_in_granularity - start_in_granularity;
                    } else {
                        if start_in_granularity != 0 {
                            partial_vec[start_index as usize] += start_in_granularity;
                        }
                        if end_in_granularity != 0 {
                            partial_vec[end_index as usize] += granularity - end_in_granularity;
                        }
                        full_adj_dif_vec[((start_index + 1) % multiple) as usize] += 1;
                        full_adj_dif_vec[(end_index as usize) - 1] -= 1;
                        if end_index < start_index {
                            full_adj_dif_vec[0] += 1;
                        }
                    }
                    full_adj_dif_vec[0] += cycle;
                }

                partial_vec
                    .into_iter()
                    .zip(full_adj_dif_vec.into_iter())
                    .map(|(partial, full)| partial + full * granularity)
                    .collect::<Vec<_>>()
            },
        )
        .collect();

    Ok(RhythmDataResponse { values: result })
}

async fn match_app_detail(
    collection: impl Iterator<Item = (AppId, Value)>,
) -> Vec<AppStatisticDetail> {
    let app_detail_map = get_all_app_detail().await;
    collection
        .map(|(app_id, value)| AppStatisticDetail {
            app: app_detail_map.get(&app_id).unwrap().clone(),
            value,
        })
        .collect()
}

fn match_category_detail(
    collection: impl Iterator<Item = (CategoryId, Value)>,
) -> Vec<CategoryStatisticDetail> {
    let category_map = get_category_detail_map();
    collection
        .map(|(category_id, value)| CategoryStatisticDetail {
            category: (&*category_map.get(&category_id).unwrap().lock().unwrap()).into(),
            value,
        })
        .collect()
}

fn get_category_and_descendants_app_ids(
    category_id: &CategoryId,
) -> Result<HashSet<AppId>, String> {
    let mut app_ids = HashSet::new();
    let category_map = get_category_detail_map();
    let category_node = category_map
        .get(category_id)
        .ok_or_else(|| format!("category not found: {}", category_id))?;
    fn get_app_ids(category: &CategoryNode, app_ids: &mut HashSet<AppId>) {
        app_ids.extend(&category.app_ids);
        for child in category.children.iter() {
            get_app_ids(&child.lock().unwrap(), app_ids);
        }
    }
    get_app_ids(&category_node.lock().unwrap(), &mut app_ids);
    Ok(app_ids)
}

fn build_category_parent_map(valid_category: Vec<String>) -> HashMap<String, String> {
    let mut parent_map = HashMap::new();
    for category in valid_category {
        let category_id = category.split_once('/').unwrap().1.to_string();
        let parent_id = category.split_once('/').unwrap().0.to_string();
        parent_map.insert(category_id, parent_id);
    }
    parent_map
}

fn check_time_span_is_days(start: Timestamp, end: Timestamp) -> bool {
    let span = end - start;
    return span % d_as_ms(1) == 0;
}

fn check_time_span_is_weeks(start: Timestamp, end: Timestamp) -> bool {
    let span = end - start;
    return span % d_as_ms(7) == 0;
}

/// Only use for distinguish is the same day
fn infer_timezone_offset(t: Timestamp) -> Timestamp {
    t % d_as_ms(1)
}
