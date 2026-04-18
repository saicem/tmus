/// Category data save periodically by timer
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};
use tracing::info;

use crate::app::constant::category_file_path;
use crate::cmd::app_detail::{get_all_app_detail, FileDetail};
use crate::util::{dump_json, load_json};

static ROOT_NODE_ID: &str = "root";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryNode {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub children: Vec<Arc<Mutex<CategoryNode>>>,
    pub app_ids: Vec<usize>,
}

impl Default for CategoryNode {
    fn default() -> Self {
        return CategoryNode {
            id: ROOT_NODE_ID.to_string(),
            parent_id: None,
            name: ROOT_NODE_ID.to_string(),
            children: Vec::new(),
            app_ids: Vec::new(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategorySimple {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
}

impl From<CategoryNode> for CategorySimple {
    fn from(node: CategoryNode) -> Self {
        Self {
            id: node.id.clone(),
            parent_id: node.parent_id.clone(),
            name: node.name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UncategorizedAppsResult {
    pub apps: Vec<FileDetail>,
    pub total: usize,
    pub has_more: bool,
}

static CATEGORY_ROOT: OnceLock<Mutex<Option<Arc<Mutex<CategoryNode>>>>> = OnceLock::new();
static APP_CATEGORY_MAP: OnceLock<Mutex<HashMap<usize, String>>> = OnceLock::new();
static CATEGORY_DETAIL_MAP: OnceLock<Mutex<HashMap<String, Arc<Mutex<CategoryNode>>>>> =
    OnceLock::new();

pub fn init() {
    let mut category_root: CategoryNode = load_json(category_file_path());
    category_root.app_ids.clear();
    let root_arc = Arc::new(Mutex::new(category_root));
    let mut app_category_map = HashMap::new();
    let mut detail_map = HashMap::new();

    build_map(
        Arc::clone(&root_arc),
        &mut app_category_map,
        &mut detail_map,
    );

    fn build_map(
        node: Arc<Mutex<CategoryNode>>,
        app_category_map: &mut HashMap<usize, String>,
        detail_map: &mut HashMap<String, Arc<Mutex<CategoryNode>>>,
    ) {
        let node_guard = node.lock().unwrap();
        for child in &node_guard.children {
            build_map(Arc::clone(child), app_category_map, detail_map);
        }
        detail_map.insert(node_guard.id.clone(), Arc::clone(&node));
        for app_id in &node_guard.app_ids {
            app_category_map.insert(*app_id, node_guard.id.clone());
        }
    }
    CATEGORY_ROOT.set(Mutex::new(Some(root_arc))).unwrap();
    APP_CATEGORY_MAP.set(Mutex::new(app_category_map)).unwrap();
    CATEGORY_DETAIL_MAP.set(Mutex::new(detail_map)).unwrap();
}

/// Check category exists before setting app category
/// If category does not exist, return error
/// If app already has a category, update it category and remove it from old category
/// If app does not have a category, add it to new category
pub fn set_app_category(app_id: usize, category_id: &str) -> Result<(), String> {
    // Check if category exists
    let detail_map = CATEGORY_DETAIL_MAP.get().unwrap().lock().unwrap();
    let category_node = detail_map
        .get(category_id)
        .ok_or("Category not found".to_string())?;

    let mut app_category_map = APP_CATEGORY_MAP.get().unwrap().lock().unwrap();

    // If app already has a category, remove it from old category
    if let Some(old_category_id) = app_category_map.get(&app_id) {
        if old_category_id == &category_id {
            return Err("category already has this app".to_string());
        }
        if let Some(old_category_node) = detail_map.get(old_category_id) {
            let mut node = old_category_node.lock().unwrap();
            node.app_ids.retain(|&id| id != app_id);
        }
    }

    // Update app_category_map
    app_category_map.insert(app_id, category_id.to_string());
    category_node.lock().unwrap().app_ids.push(app_id);
    Ok(())
}

/// Remove app from category
/// If app does not have a category, do nothing
/// If app has a category, remove it from category
pub fn remove_app_from_category(app_id: usize) {
    let state = APP_CATEGORY_MAP.get().unwrap();
    let mut data = state.lock().unwrap();
    if let Some(category_id) = data.remove(&app_id) {
        let detail_map = CATEGORY_DETAIL_MAP.get().unwrap().lock().unwrap();
        if let Some(category_node) = detail_map.get(&category_id) {
            let mut node = category_node.lock().unwrap();
            node.app_ids.retain(|&id| id != app_id);
        }
        drop(detail_map);
    }
}

pub fn get_category_all() -> CategoryNode {
    get_node_self(&ROOT_NODE_ID).unwrap().lock().unwrap().clone()
}

/// Find node from category map by parent id
/// If parent id does not exist, return None
/// If parent id exists, return CategorySimple
/// Add node to parent node find before add to detail map
pub fn add_category(parent_id: String, name: String) -> Result<CategorySimple, String> {
    // Check if parent exists
    let mut detail_map = CATEGORY_DETAIL_MAP.get().unwrap().lock().unwrap();
    let parent_node = detail_map
        .get(&parent_id)
        .ok_or("Parent category not found".to_string())?;
    let category_id = uuid::Uuid::new_v4().to_string();

    // Create new category node
    let new_category = Arc::new(Mutex::new(CategoryNode {
        id: category_id.clone(),
        parent_id: Some(parent_id.clone()),
        name: name.clone(),
        children: Vec::new(),
        app_ids: Vec::new(),
    }));

    parent_node
        .lock()
        .unwrap()
        .children
        .push(new_category.clone());
    detail_map.insert(category_id.clone(), new_category);

    Ok(CategorySimple {
        id: category_id,
        parent_id: Some(parent_id),
        name,
    })
}

pub fn rename_category(id: String, name: String) -> Result<(), String> {
    let arc_node = get_node_self(&id).ok_or("Category not found".to_string())?;
    let mut node = arc_node.lock().unwrap();
    node.name = name;
    Ok(())
}

pub fn delete_category(id: String) -> Result<(), String> {
    if id == ROOT_NODE_ID {
        return Err("Cannot delete root category".to_string());
    }
    let arc_node = get_node_self(&id).ok_or("Category not found".to_string())?;
    let node = arc_node.lock().unwrap();
    let parent_node = get_node_self(&node.parent_id.as_ref().unwrap())
        .ok_or("Parent category not found".to_string())?;
    let mut parent_node = parent_node.lock().unwrap();
    parent_node
        .children
        .retain(|child| !Arc::ptr_eq(child, &arc_node));
    let (node_ids, app_ids) = get_node_ids_all(&node);

    let mut detail_map = CATEGORY_DETAIL_MAP.get().unwrap().lock().unwrap();
    for k in &node_ids {
        detail_map.remove(k);
    }
    drop(detail_map);

    let mut app_map = APP_CATEGORY_MAP.get().unwrap().lock().unwrap();
    app_ids.iter().for_each(|k| {
        app_map.remove(k);
    });
    drop(app_map);
    Ok(())
}

pub fn get_app_category(app_id: usize) -> Result<CategorySimple, String> {
    let state = APP_CATEGORY_MAP.get().unwrap();
    let data = state.lock().unwrap();
    let id = data.get(&app_id).ok_or("App not found".to_string())?;
    let detail_map = CATEGORY_DETAIL_MAP.get().unwrap().lock().unwrap();
    let node_arc = detail_map.get(id).ok_or("Category not found".to_string())?;
    let node = node_arc.lock().unwrap();
    Ok(node.clone().into())
}

pub async fn get_uncategorized_apps(
    offset: usize,
    limit: usize,
    keyword: Option<String>,
) -> Result<UncategorizedAppsResult, String> {
    let categorized_apps: HashSet<usize> = APP_CATEGORY_MAP
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .keys()
        .copied()
        .collect();

    let all_apps = get_all_app_detail().await;
    let mut uncategorized = Vec::new();
    let keyword_lower = keyword.as_ref().map(|k| k.to_lowercase());

    for detail in all_apps.iter() {
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
fn get_node_self(id: &str) -> Option<Arc<Mutex<CategoryNode>>> {
    let detail_map = CATEGORY_DETAIL_MAP.get().unwrap().lock().unwrap();
    detail_map.get(id).cloned()
}

/// Find all children node and this node self ids and app ids
fn get_node_ids_all(node: &CategoryNode) -> (Vec<String>, Vec<usize>) {
    let mut node_ids = Vec::new();
    let mut app_ids = Vec::new();
    fn dfs(node: &CategoryNode, node_ids: &mut Vec<String>, app_ids: &mut Vec<usize>) {
        node_ids.push(node.id.clone());
        app_ids.extend(&node.app_ids);
        for child in &node.children {
            dfs(&child.lock().unwrap(), node_ids, app_ids);
        }
    }
    dfs(node, &mut node_ids, &mut app_ids);
    (node_ids, app_ids)
}
