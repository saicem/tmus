use serde::Serialize;

use crate::cli::output::{format_duration, print_json, print_table};
use crate::cli::{now_timestamp, parse_timestamp, AppCommand, UsageArgs};
use crate::cmd::app_detail::{get_all_app_detail, FileDetail};
use crate::cmd::duration::duration_by_id;
use crate::state::category::{get_app_category, save_category_data, set_app_category, CategoryId};
use crate::util::FileVersion;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppListItem {
    id: usize,
    name: String,
    path: String,
    exist: bool,
    category_id: Option<CategoryId>,
    category_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<FileVersion>,
}

impl AppListItem {
    fn from_detail(detail: &FileDetail) -> Self {
        let category = get_app_category(detail.id).ok();
        Self {
            id: detail.id,
            name: detail.name.clone(),
            path: detail.path.clone(),
            exist: detail.exist,
            category_id: category.as_ref().map(|c| c.id),
            category_name: category.as_ref().map(|c| c.name.clone()),
            version: detail.version.clone(),
        }
    }
}

pub async fn handle(cmd: AppCommand, json: bool) -> Result<(), String> {
    match cmd {
        AppCommand::List => list(json).await,
        AppCommand::Detail { app_id } => detail(app_id, json).await,
        AppCommand::Category { app_id } => get_category(app_id, json),
        AppCommand::SetCategory { category, app_ids } => set_category(category, &app_ids, json),
        AppCommand::Uncategorized {
            offset,
            limit,
            keyword,
        } => uncategorized(offset, limit, keyword, json).await,
    }
}

async fn list(json: bool) -> Result<(), String> {
    let app_detail_map = get_all_app_detail().await;
    let mut details: Vec<&FileDetail> = app_detail_map.values().collect();
    details.sort_by_key(|d| d.id);

    if json {
        let items: Vec<AppListItem> = details
            .iter()
            .map(|d| AppListItem::from_detail(d))
            .collect();
        return print_json(&items);
    }

    let rows: Vec<Vec<String>> = details
        .iter()
        .map(|d| {
            let category = get_app_category(d.id)
                .ok()
                .map(|c| c.name)
                .unwrap_or_default();
            vec![
                d.id.to_string(),
                d.name.clone(),
                d.path.clone(),
                category,
                if d.exist { "yes" } else { "no" }.to_string(),
            ]
        })
        .collect();
    print_table(&["ID", "Name", "Path", "Category", "Exists"], &rows);
    Ok(())
}

async fn detail(app_id: usize, json: bool) -> Result<(), String> {
    let app_detail_map = get_all_app_detail().await;
    let detail = app_detail_map
        .get(&app_id)
        .ok_or_else(|| format!("App {app_id} not found"))?;

    if json {
        return print_json(detail);
    }

    println!("ID: {}", detail.id);
    println!("Name: {}", detail.name);
    println!("Path: {}", detail.path);
    println!("Exists: {}", detail.exist);
    if let Some(version) = &detail.version {
        if let Some(product_name) = &version.product_name {
            println!("Product Name: {product_name}");
        }
        if let Some(company_name) = &version.company_name {
            println!("Company: {company_name}");
        }
        if let Some(description) = &version.file_description {
            println!("Description: {description}");
        }
        if let Some(file_version) = &version.file_version {
            println!("File Version: {file_version}");
        }
        if let Some(product_version) = &version.product_version {
            println!("Product Version: {product_version}");
        }
    }
    match get_app_category(app_id) {
        Ok(category) => println!("Category: {} [id={}]", category.name, category.id),
        Err(_) => println!("Category: (none)"),
    }
    Ok(())
}

fn get_category(app_id: usize, json: bool) -> Result<(), String> {
    let category = get_app_category(app_id).ok();
    if json {
        return print_json(&category);
    }
    match category {
        Some(category) => println!(
            "App {app_id} category: {} [id={}]",
            category.name, category.id
        ),
        None => println!("App {app_id} has no category"),
    }
    Ok(())
}

fn set_category(category_id: CategoryId, app_ids: &[usize], json: bool) -> Result<(), String> {
    let mut changed = false;
    for &app_id in app_ids {
        match set_app_category(app_id, category_id) {
            Ok(()) => changed = true,
            // Idempotent: the app is already in the target category.
            Err(e) if e.contains("already has this app") => {}
            Err(e) => return Err(format!("Failed to set app {app_id}: {e}")),
        }
    }
    if changed {
        save_category_data().map_err(|e| format!("Failed to save category data: {e}"))?;
    }
    if json {
        return print_json(&serde_json::json!({
            "success": true,
            "categoryId": category_id,
            "appIds": app_ids,
        }));
    }
    println!("Set category {category_id} for {} app(s)", app_ids.len());
    Ok(())
}

async fn uncategorized(
    offset: usize,
    limit: usize,
    keyword: Option<String>,
    json: bool,
) -> Result<(), String> {
    let result = crate::state::category::get_uncategorized_apps(offset, limit, keyword).await?;
    if json {
        return print_json(&serde_json::json!({
            "apps": result.apps,
            "total": result.total,
            "hasMore": result.has_more,
        }));
    }
    let rows: Vec<Vec<String>> = result
        .apps
        .iter()
        .map(|d| vec![d.id.to_string(), d.name.clone(), d.path.clone()])
        .collect();
    print_table(&["ID", "Name", "Path"], &rows);
    println!("Total: {} (showing {})", result.total, result.apps.len());
    Ok(())
}

pub async fn usage(args: UsageArgs, json: bool) -> Result<(), String> {
    let end = args
        .end
        .as_deref()
        .map(parse_timestamp)
        .transpose()?
        .unwrap_or_else(now_timestamp);
    let start = match &args.start {
        Some(s) => parse_timestamp(s)?,
        None if args.last_month => now_timestamp() - 30 * 24 * 60 * 60 * 1000,
        None => 0,
    };
    if start >= end {
        return Err("Start time must be before end time".to_string());
    }

    let mut durations = duration_by_id(start, end);
    durations.sort_by_key(|x| std::cmp::Reverse(x.duration));
    if let Some(top_k) = args.top_k {
        durations.truncate(top_k);
    }

    let app_detail_map = get_all_app_detail().await;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct UsageItem {
        id: usize,
        name: String,
        path: String,
        duration: i64,
        duration_text: String,
        category_id: Option<CategoryId>,
        category_name: Option<String>,
    }

    let mut items: Vec<UsageItem> = Vec::new();
    let mut total_duration: i64 = 0;
    for id_duration in durations {
        if let Some(detail) = app_detail_map.get(&id_duration.app_id) {
            let category = get_app_category(detail.id).ok();
            total_duration += id_duration.duration;
            items.push(UsageItem {
                id: detail.id,
                name: detail.name.clone(),
                path: detail.path.clone(),
                duration: id_duration.duration,
                duration_text: format_duration(id_duration.duration),
                category_id: category.as_ref().map(|c| c.id),
                category_name: category.as_ref().map(|c| c.name.clone()),
            });
        }
    }

    if json {
        return print_json(&items);
    }

    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            vec![
                item.id.to_string(),
                item.name.clone(),
                format_duration(item.duration),
                item.category_name.clone().unwrap_or_default(),
            ]
        })
        .collect();
    print_table(&["ID", "Name", "Duration", "Category"], &rows);
    println!(
        "Total duration in range: {}",
        format_duration(total_duration)
    );
    Ok(())
}
