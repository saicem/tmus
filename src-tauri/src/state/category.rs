/// Category data save periodically by timer
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use tmus_engine::models::AppId;
use tracing::info;

use crate::app::constant::category_file_path;
use crate::cmd::app_detail::{get_all_app_detail, FileDetail};
use crate::util::{dump_json, load_json};

pub type CategoryId = u64;

pub const ROOT_NODE_ID: CategoryId = 0;
pub const UNCATEGORIZED_CATEGORY_ID: CategoryId = CategoryId::MAX;
static NEXT_CATEGORY_ID: AtomicU64 = AtomicU64::new(0);
static CATEGORY_ROOT: OnceLock<Arc<Mutex<CategoryNode>>> = OnceLock::new();
static APP_CATEGORY_MAP: OnceLock<Mutex<HashMap<AppId, CategoryId>>> = OnceLock::new();
static CATEGORY_DETAIL_MAP: OnceLock<Mutex<HashMap<CategoryId, Arc<Mutex<CategoryNode>>>>> =
    OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryNode {
    pub id: CategoryId,
    pub parent_id: CategoryId,
    pub name: String,
    pub children: Vec<Arc<Mutex<CategoryNode>>>,
    pub app_ids: Vec<AppId>,
}

impl Default for CategoryNode {
    fn default() -> Self {
        return CategoryNode {
            id: ROOT_NODE_ID,
            parent_id: ROOT_NODE_ID,
            name: "ROOT".to_string(),
            children: Vec::new(),
            app_ids: Vec::new(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategorySimple {
    pub id: CategoryId,
    pub parent_id: CategoryId,
    pub name: String,
}

impl From<&CategoryNode> for CategorySimple {
    fn from(node: &CategoryNode) -> Self {
        Self {
            id: node.id.to_owned(),
            parent_id: node.parent_id.to_owned(),
            name: node.name.to_owned(),
        }
    }
}

impl From<CategoryNode> for CategorySimple {
    fn from(node: CategoryNode) -> Self {
        Self {
            id: node.id,
            parent_id: node.parent_id,
            name: node.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UncategorizedAppsResult {
    pub apps: Vec<FileDetail>,
    pub total: usize,
    pub has_more: bool,
}

#[inline]
pub fn get_app_category_map<'a>() -> MutexGuard<'a, HashMap<AppId, CategoryId>> {
    APP_CATEGORY_MAP.get().unwrap().lock().unwrap()
}

#[inline]
pub fn get_category_detail_map<'a>() -> MutexGuard<'a, HashMap<CategoryId, Arc<Mutex<CategoryNode>>>>
{
    CATEGORY_DETAIL_MAP.get().unwrap().lock().unwrap()
}

pub fn init() {
    let mut category_root: CategoryNode = load_json(category_file_path());
    category_root.app_ids.clear();

    let root_arc = Arc::new(Mutex::new(category_root));
    let mut app_category_map = HashMap::new();
    let mut detail_map = HashMap::new();

    build_map(&root_arc, &mut app_category_map, &mut detail_map);
    fn build_map(
        node_mutex: &Arc<Mutex<CategoryNode>>,
        app_category_map: &mut HashMap<usize, CategoryId>,
        detail_map: &mut HashMap<CategoryId, Arc<Mutex<CategoryNode>>>,
    ) {
        let node = node_mutex.lock().unwrap();
        detail_map.insert(node.id, Arc::clone(&node_mutex));
        for child in &node.children {
            build_map(child, app_category_map, detail_map);
        }
        for app_id in &node.app_ids {
            app_category_map.insert(*app_id, node.id.to_owned());
        }
    }

    let mut cur_id = ROOT_NODE_ID;
    reset_id(&root_arc, &mut cur_id, ROOT_NODE_ID);
    fn reset_id(
        node_mutex: &Arc<Mutex<CategoryNode>>,
        cur_id: &mut CategoryId,
        parent_id: CategoryId,
    ) {
        let mut node = node_mutex.lock().unwrap();
        node.id = *cur_id;
        node.parent_id = parent_id;
        for child in &node.children {
            *cur_id += 1;
            reset_id(child, cur_id, node.id);
        }
    }

    NEXT_CATEGORY_ID.store(detail_map.len() as CategoryId, Ordering::Relaxed);
    CATEGORY_ROOT.set(root_arc).unwrap();
    APP_CATEGORY_MAP.set(Mutex::new(app_category_map)).unwrap();
    CATEGORY_DETAIL_MAP.set(Mutex::new(detail_map)).unwrap();
}

/// Check category exists before setting app category
/// If category does not exist, return error
/// If app already has a category, update it category and remove it from old category
/// If app does not have a category, add it to new category
pub fn set_app_category(app_id: AppId, category_id: CategoryId) -> Result<(), String> {
    // Check if category exists
    let detail_map = get_category_detail_map();
    let category_node_arc_mutex = detail_map
        .get(&category_id)
        .ok_or("Category not found".to_string())?
        .clone();

    let mut app_category_map = get_app_category_map();

    // If app already has a category, remove it from old category
    if let Some(old_category_id) = app_category_map.get(&app_id) {
        if *old_category_id == category_id {
            return Err("category already has this app".to_string());
        }
        if let Some(old_category_node) = detail_map.get(old_category_id) {
            let mut node = old_category_node.lock().unwrap();
            node.app_ids.retain(|&id| id != app_id);
        }
    }

    category_node_arc_mutex.lock().unwrap().app_ids.push(app_id);
    app_category_map.insert(app_id, category_id);
    Ok(())
}

/// Remove app from category
/// If app does not have a category, do nothing
/// If app has a category, remove it from category
pub fn remove_app_from_category(app_id: AppId) {
    let mut app_category_map = get_app_category_map();
    if let Some(category_id) = app_category_map.remove(&app_id) {
        let detail_map = get_category_detail_map();
        if let Some(category_node) = detail_map.get(&category_id) {
            let mut node = category_node.lock().unwrap();
            node.app_ids.retain(|&id| id != app_id);
        }
        drop(detail_map);
    }
}

pub fn get_category_tree() -> Arc<Mutex<CategoryNode>> {
    CATEGORY_ROOT.get().unwrap().clone()
}

/// Find node from category map by parent id
/// If parent id does not exist, return None
/// If parent id exists, return CategorySimple
/// Add node to parent node find before add to detail map
pub fn add_category(parent_id: CategoryId, name: String) -> Result<(), String> {
    // Check if parent exists
    let mut detail_map = get_category_detail_map();
    let parent_node_mutex = detail_map
        .get(&parent_id)
        .ok_or("Parent category not found".to_string())?;
    let mut parent_node = parent_node_mutex.lock().unwrap();
    let category_id = generate_category_id();

    // Create new category node
    let new_category = Arc::new(Mutex::new(CategoryNode {
        id: category_id.clone(),
        parent_id: parent_id,
        name: name.clone(),
        children: Vec::new(),
        app_ids: Vec::new(),
    }));

    parent_node.children.push(new_category.clone());
    drop(parent_node);
    detail_map.insert(category_id.clone(), new_category);
    Ok(())
}

pub fn rename_category(id: CategoryId, name: String) -> Result<(), String> {
    let arc_node = get_node_self(id).ok_or("Category not found".to_string())?;
    let mut node = arc_node.lock().unwrap();
    node.name = name;
    Ok(())
}

pub fn delete_category(id: CategoryId) -> Result<(), String> {
    if id == ROOT_NODE_ID {
        return Err("Cannot delete root category".to_string());
    }
    let arc_node = get_node_self(id).ok_or("Category not found".to_string())?;
    let node = arc_node.lock().unwrap();
    let parent_node =
        get_node_self(node.parent_id).ok_or("Parent category not found".to_string())?;
    let mut parent_node = parent_node.lock().unwrap();
    parent_node
        .children
        .retain(|child| !Arc::ptr_eq(child, &arc_node));
    let (node_ids, app_ids) = get_self_and_descendents_ids(&node);

    let mut detail_map = get_category_detail_map();
    for node_id in &node_ids {
        detail_map.remove(node_id);
    }
    drop(detail_map);

    let mut app_category_map = get_app_category_map();
    app_ids.iter().for_each(|k| {
        app_category_map.remove(k);
    });
    drop(app_category_map);
    Ok(())
}

pub fn get_app_category(app_id: AppId) -> Result<CategorySimple, String> {
    let app_category_map = get_app_category_map();
    let id = app_category_map
        .get(&app_id)
        .ok_or("App not found".to_string())?;
    let detail_map = get_category_detail_map();
    let node_arc = detail_map.get(id).ok_or("Category not found".to_string())?;
    let node = node_arc.lock().unwrap();
    Ok((&*node).into())
}

pub async fn get_uncategorized_apps(
    offset: usize,
    limit: usize,
    keyword: Option<String>,
) -> Result<UncategorizedAppsResult, String> {
    let categorized_apps: HashSet<usize> = get_app_category_map().keys().copied().collect();
    let app_detail_map = get_all_app_detail().await;
    let mut uncategorized = Vec::new();
    let keyword_lower = keyword.as_ref().map(|k| k.to_lowercase());

    for detail in app_detail_map.values() {
        if categorized_apps.contains(&detail.id) {
            continue;
        }
        if keyword_lower
            .as_ref()
            .map(|kw| !match_app_detail(detail, kw))
            .unwrap_or(false)
        {
            continue;
        }
        uncategorized.push(detail.clone());
    }

    let total = uncategorized.len();
    let has_more = total > offset + limit;

    uncategorized = if has_more {
        uncategorized[offset..offset + limit].to_vec()
    } else {
        uncategorized[offset..total].to_vec()
    };

    Ok(UncategorizedAppsResult {
        apps: uncategorized,
        total,
        has_more,
    })
}

pub fn save_category_data() {
    dump_json(CATEGORY_ROOT.get().unwrap(), category_file_path());
    info!("Category data saved");
}

pub fn get_category_self_and_descendants_map() -> HashMap<CategoryId, HashSet<CategoryId>> {
    let root = get_category_tree();
    let mut descendants_map = HashMap::new();
    get_self_and_descendants(&root.lock().unwrap(), &mut descendants_map);
    fn get_self_and_descendants<'a>(
        category: &CategoryNode,
        descendants_map: &'a mut HashMap<CategoryId, HashSet<CategoryId>>,
    ) -> &'a HashSet<CategoryId> {
        let mut set = HashSet::from([category.id]);
        for child in category.children.iter() {
            let child = child.lock().unwrap();
            let descendants = get_self_and_descendants(&child, descendants_map);
            set.extend(descendants);
        }
        descendants_map.insert(category.id, set);
        descendants_map.get(&category.id).unwrap()
    }
    descendants_map
}

/// Match app detail with keyword
/// If app path contains keyword, return true
/// If app file description contains keyword, return true
/// If app company name contains keyword, return true
/// Otherwise, return false
fn match_app_detail(detail: &FileDetail, keyword: &str) -> bool {
    if detail.path.contains(keyword) {
        return true;
    }
    if let Some(version) = &detail.version {
        return version
            .file_description
            .as_ref()
            .map(|s| s.contains(keyword))
            .unwrap_or(false)
            || version
                .company_name
                .as_ref()
                .map(|s| s.contains(keyword))
                .unwrap_or(false);
    }
    false
}

/// Get node by id
fn get_node_self(id: CategoryId) -> Option<Arc<Mutex<CategoryNode>>> {
    let detail_map = get_category_detail_map();
    detail_map.get(&id).cloned()
}

/// Find node self and descendants node ids and app ids
fn get_self_and_descendents_ids(node: &CategoryNode) -> (Vec<CategoryId>, Vec<AppId>) {
    let mut node_ids = Vec::new();
    let mut app_ids = Vec::new();
    fn dfs(node: &CategoryNode, node_ids: &mut Vec<CategoryId>, app_ids: &mut Vec<AppId>) {
        node_ids.push(node.id.clone());
        app_ids.extend(&node.app_ids);
        for child in &node.children {
            dfs(&child.lock().unwrap(), node_ids, app_ids);
        }
    }
    dfs(node, &mut node_ids, &mut app_ids);
    (node_ids, app_ids)
}

fn generate_category_id() -> CategoryId {
    NEXT_CATEGORY_ID.fetch_add(1, Ordering::Relaxed)
}
