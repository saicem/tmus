use crate::{
    cmd::{
        app_detail::{get_all_app_detail, FileDetail},
        read_helper,
    },
    state::category::{
        get_app_category_map, get_category_detail_map, get_category_self_and_descendants_map,
        CategoryId, CategoryNode, CategorySimple, UNCATEGORIZED_CATEGORY_ID,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};
use tmus_engine::models::FocusRecord;
use tmus_engine::{
    models::AppId,
    util::{d_as_ms, ms_as_d, Timestamp},
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
    pub category_id: Option<CategoryId>,
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
    /// Group<Granularity<AverageDuration>>
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
    let mut detail = match_app_detail(map.into_iter().map(|(app_id, value)| (app_id, value))).await;
    detail.sort_by_key(|x| -x.value);
    Ok(AppDurationResponse { detail })
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
    let mut detail = match_app_detail(
        map.into_iter()
            .map(|(app_id, set)| (app_id, set.len() as i64)),
    )
    .await;
    detail.sort_by_key(|x| -x.value);
    Ok(AppDayCountResponse { detail })
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
    let include_uncategorized_category = request.category_ids.contains(&UNCATEGORIZED_CATEGORY_ID);
    for category_id in request.category_ids {
        if let Some(self_descendants_category_ids) =
            category_self_and_descendants_map.get(&category_id)
        {
            let value = self_descendants_category_ids
                .iter()
                .map(|id| categorized_duration_map.get(id).unwrap_or(&0))
                .sum::<i64>();
            category_with_descendants_duration_map.insert(category_id, value);
        }
    }

    let mut detail = match_category_detail(
        category_with_descendants_duration_map
            .into_iter()
            .map(|(category_id, value)| (category_id, value)),
    );

    if include_uncategorized_category {
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
    let include_uncategorized_category = request.category_ids.contains(&UNCATEGORIZED_CATEGORY_ID);
    for category_id in request.category_ids {
        if let Some(self_descendants_category_ids) = category_and_descendants_map.get(&category_id)
        {
            let mut set: HashSet<i64> = HashSet::new();
            self_descendants_category_ids
                .iter()
                .filter_map(|id| categorized_days_map.get(id))
                .for_each(|category_set| set.extend(category_set));
            category_with_descendants_days_count_map.insert(category_id, set.len());
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
    let granularity_multiple = time_span / granularity;
    if granularity_multiple > 24 * 60 {
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
            return Err("end_time - start_time must be a multiple of span".to_string());
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
                let time_span_multiple = (end_time - start_time) / time_span;
                let filter_records: Box<dyn Fn(&FocusRecord) -> bool> = {
                    if category_id.is_none() {
                        Box::new(move |_record: &FocusRecord| -> bool { true })
                    } else {
                        let category_id = category_id.unwrap();
                        let valid_app_ids = get_category_and_descendants_app_ids(&category_id);
                        if valid_app_ids.is_err() {
                            tracing::error!(
                                "get_category_and_descendants_app_ids error: {:?}",
                                category_id
                            );
                            Box::new(move |_record: &FocusRecord| -> bool { false })
                        } else {
                            let valid_app_ids = valid_app_ids.unwrap();
                            Box::new(move |record: &FocusRecord| -> bool { valid_app_ids.contains(&record.id) })
                        }
                    }
                };
                let mut ret = vec![0; granularity_multiple as usize];
                let records = read_helper::read_by_timestamp(start_time, end_time);
                if records.is_empty() {
                    return ret;
                }
                let mut cur_index = -1;
                let mut cur_bound = 0;

                for record in records {
                    if !filter_records(&record) {
                        continue;
                    }
                    let mut start = record.focus_at;
                    let end = record.blur_at;
                    if start >= cur_bound {
                        (cur_index, cur_bound) = {
                            let start_tz = record.focus_at + timezone_offset;
                            let cur_index = (start_tz / granularity) % granularity_multiple;
                            let cur_bound =
                                (start_tz / granularity + 1) * granularity - timezone_offset;
                            (cur_index, cur_bound)
                        }
                    }
                    while start < end {
                        if end < cur_bound {
                            ret[cur_index as usize] += end - start;
                            break;
                        } else {
                            ret[cur_index as usize] += cur_bound - start;
                            start = cur_bound;
                            cur_bound += granularity;
                            cur_index = (cur_index + 1) % granularity_multiple;
                        }
                    }
                }
                ret.iter_mut().for_each(|v| *v /= time_span_multiple);
                ret
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

/// Only use for distinguish is the same day
fn infer_timezone_offset(t: Timestamp) -> Timestamp {
    -(t % d_as_ms(1))
}
