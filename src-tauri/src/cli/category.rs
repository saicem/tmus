use std::sync::{Arc, Mutex};

use crate::cli::output::{print_json, print_table};
use crate::cli::CategoryCommand;
use crate::cmd::app_detail::{get_all_app_detail, FileDetail};
use crate::state::category::{
    self, get_category_detail_map, save_category_data, CategoryId, CategoryNode,
};

pub async fn handle(cmd: CategoryCommand, json: bool) -> Result<(), String> {
    match cmd {
        CategoryCommand::List => list(json),
        CategoryCommand::Apps { id } => apps(id, json).await,
        CategoryCommand::Add { name, parent } => add(name, parent, json),
        CategoryCommand::Rename { id, name } => rename(id, name, json),
        CategoryCommand::Delete { id } => delete(id, json),
    }
}

fn list(json: bool) -> Result<(), String> {
    let root = category::get_category_tree();
    let tree = root
        .lock()
        .map_err(|_| "Category state poisoned".to_string())?
        .clone();

    if json {
        return print_json(&tree);
    }

    let mut lines = Vec::new();
    for child in &tree.children {
        collect_tree_lines(child, 0, &mut lines)?;
    }
    for line in lines {
        println!("{line}");
    }
    Ok(())
}

fn collect_tree_lines(
    node: &Arc<Mutex<CategoryNode>>,
    depth: usize,
    lines: &mut Vec<String>,
) -> Result<(), String> {
    let node = node
        .lock()
        .map_err(|_| "Category state poisoned".to_string())?;
    let indent = "  ".repeat(depth);
    lines.push(format!(
        "{indent}{} [id={}, apps={}]",
        node.name,
        node.id,
        node.app_ids.len()
    ));
    for child in &node.children {
        collect_tree_lines(child, depth + 1, lines)?;
    }
    Ok(())
}

async fn apps(category_id: CategoryId, json: bool) -> Result<(), String> {
    let detail_map = get_all_app_detail().await;
    let node = get_category_detail_map()
        .get(&category_id)
        .cloned()
        .ok_or_else(|| format!("Category {category_id} not found"))?;
    let app_ids = node
        .lock()
        .map_err(|_| "Category state poisoned".to_string())?
        .app_ids
        .clone();

    let mut details: Vec<&FileDetail> = app_ids
        .iter()
        .filter_map(|id| detail_map.get(id))
        .collect();
    details.sort_by_key(|d| d.id);

    if json {
        return print_json(&details);
    }

    let rows: Vec<Vec<String>> = details
        .iter()
        .map(|d| vec![d.id.to_string(), d.name.clone(), d.path.clone()])
        .collect();
    print_table(&["ID", "Name", "Path"], &rows);
    Ok(())
}

fn add(name: String, parent: CategoryId, json: bool) -> Result<(), String> {
    category::add_category(parent, name.clone())?;
    persist()?;
    if json {
        return print_json(&serde_json::json!({
            "success": true,
            "name": name,
            "parentId": parent,
        }));
    }
    println!("Category \"{name}\" added under parent {parent}");
    Ok(())
}

fn rename(id: CategoryId, name: String, json: bool) -> Result<(), String> {
    category::rename_category(id, name.clone())?;
    persist()?;
    if json {
        return print_json(&serde_json::json!({
            "success": true,
            "id": id,
            "name": name,
        }));
    }
    println!("Category {id} renamed to \"{name}\"");
    Ok(())
}

fn delete(id: CategoryId, json: bool) -> Result<(), String> {
    category::delete_category(id)?;
    persist()?;
    if json {
        return print_json(&serde_json::json!({
            "success": true,
            "id": id,
        }));
    }
    println!("Category {id} deleted");
    Ok(())
}

fn persist() -> Result<(), String> {
    save_category_data().map_err(|e| format!("Failed to save category data: {e}"))
}
