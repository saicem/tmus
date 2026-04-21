use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex, MutexGuard, OnceLock,
};

use crate::{
    app::constant::statistic_scheme_file_path,
    cmd::statistic::{
        AppDayCountRequest, AppDurationRequest, CategoryDayCountRequest, CategoryDurationRequest,
        RhythmRequest,
    },
    util::{dump_json, load_json},
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum StatisticSchemeDetail {
    AppDurationRequest(AppDurationRequest),
    AppDayCountRequest(AppDayCountRequest),
    CategoryDurationRequest(CategoryDurationRequest),
    CategoryDayCountRequest(CategoryDayCountRequest),
    RhythmRequest(RhythmRequest),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatisticSchemeItem {
    pub id: u64,
    pub name: String,
    pub detail: StatisticSchemeDetail,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatisticScheme {
    pub items: Vec<StatisticSchemeItem>,
}

static STATISTIC_SCHEME: OnceLock<Arc<Mutex<StatisticScheme>>> = OnceLock::new();
static STATISTIC_SCHEME_CHANGED: AtomicBool = AtomicBool::new(false);
static STATISTIC_SCHEME_NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub fn get_statistic_scheme<'a>() -> MutexGuard<'a, StatisticScheme> {
    STATISTIC_SCHEME
        .get_or_init(|| {
            let mut scheme: StatisticScheme = load_json(statistic_scheme_file_path());
            // Reset id
            scheme.items.iter_mut().enumerate().for_each(|(i, item)| {
                item.id = i as u64;
            });
            STATISTIC_SCHEME_NEXT_ID.store(scheme.items.len() as u64, Ordering::Relaxed);
            Arc::new(Mutex::new(scheme))
        })
        .lock()
        .unwrap()
}

pub fn save_statistic_scheme() {
    if !STATISTIC_SCHEME_CHANGED.swap(false, Ordering::Relaxed) {
        return;
    }
    let scheme = get_statistic_scheme();
    dump_json(&*scheme, statistic_scheme_file_path());
    info!("Saved statistic scheme");
}

pub fn add_statistic_scheme_item(name: String, detail: StatisticSchemeDetail) {
    let mut scheme = get_statistic_scheme();
    scheme.items.push(StatisticSchemeItem {
        id: STATISTIC_SCHEME_NEXT_ID.fetch_add(1, Ordering::Relaxed),
        name,
        detail,
    });
    STATISTIC_SCHEME_CHANGED.store(true, Ordering::Relaxed);
}

pub fn delete_statistic_scheme_item(id: &u64) -> Result<(), String> {
    {
        static LOCK: Mutex<()> = Mutex::new(());
        let _lock = LOCK.lock().unwrap();
        let mut scheme = get_statistic_scheme();
        let index = scheme
            .items
            .binary_search_by_key(id, |item| item.id)
            .map_err(|_| "Item not found".to_string())?;
        scheme.items.remove(index);
    }
    STATISTIC_SCHEME_CHANGED.store(true, Ordering::Relaxed);
    Ok(())
}
