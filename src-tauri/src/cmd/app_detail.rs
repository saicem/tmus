use crate::app::constant::app_detail_cache_path;
use crate::util;
use crate::util::{dump_json, load_json, FileVersion};
use base64::engine::general_purpose;
use base64::Engine;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::sync::OnceLock;
use tmus_engine::storage::focus_app;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub id: usize,
    pub name: String,
    pub path: String,
    pub exist: bool,
    pub icon: Option<String>,
    pub version: Option<FileVersion>,
}

pub fn get_app_detail_cache<'a>() -> &'a Mutex<HashMap<usize, FileDetail>> {
    static APP_DETAIL_CACHE: OnceLock<Mutex<HashMap<usize, FileDetail>>> = OnceLock::new();
    APP_DETAIL_CACHE.get_or_init(|| {
        let details: Vec<FileDetail> = load_json(app_detail_cache_path());
        Mutex::new(
            details
                .into_iter()
                .map(|detail| (detail.id, detail))
                .collect(),
        )
    })
}

pub async fn update_app_detail_cache(
    app_detail_map: &mut HashMap<usize, FileDetail>,
    details: Vec<FileDetail>,
) {
    for detail in details {
        app_detail_map.insert(detail.id, detail);
    }
    dump_json(
        &app_detail_map
            .values()
            .map(|x| x.to_owned())
            .collect::<Vec<FileDetail>>(),
        app_detail_cache_path(),
    );
}

#[tauri::command]
#[tracing::instrument]
pub async fn get_app_detail(id: usize) -> FileDetail {
    let path = focus_app::get_path_by_id(id);
    let detail = query_file_detail(id, &path);
    let mut app_detail_cache = get_app_detail_cache().lock().await;
    update_app_detail_cache(&mut *app_detail_cache, vec![detail.clone()]).await;
    detail
}

#[tauri::command]
#[tracing::instrument]
pub async fn get_all_app_detail() -> Vec<FileDetail> {
    let app_vec = focus_app::get_all_app();
    let mut app_detail_cache = get_app_detail_cache().lock().await;
    app_detail_cache.values_mut().for_each(|detail| {
        detail.exist = Path::new(&detail.path).exists();
    });
    let not_exist_app_detail: Vec<FileDetail> = app_vec
        .iter()
        .enumerate()
        .filter(|(id, _)| !app_detail_cache.contains_key(id))
        .map(|(id, path)| query_file_detail(id, path))
        .collect();
    if not_exist_app_detail.len() > 0 {
        update_app_detail_cache(&mut *app_detail_cache, not_exist_app_detail).await;
    }
    app_detail_cache.values().map(|x| x.to_owned()).collect()
}

fn query_file_detail(id: usize, path: &str) -> FileDetail {
    let extract_name_from_path = |path: &str| {
        Path::new(&path)
            .file_stem()
            .map(|file_stem| file_stem.to_str())
            .flatten()
            .map_or_else(|| path.to_owned(), |file_stem| file_stem.to_owned())
    };
    if !Path::new(&path).exists() {
        return FileDetail {
            name: extract_name_from_path(&path),
            id,
            path: path.to_owned(),
            exist: false,
            icon: None,
            version: None,
        };
    }
    let version = util::get_file_version(&path);
    let icon = util::extract_icon(&path).map(|x| {
        let mut buf = Vec::new();
        x.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .unwrap();
        format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(&buf)
        )
    });
    let name: String = match &version {
        Some(FileVersion {
            product_name: Some(name),
            ..
        }) if !name.is_empty() && name.ne("Microsoft® Windows® Operating System") => {
            name.to_owned()
        }
        Some(FileVersion {
            file_description: Some(name),
            ..
        }) if !name.is_empty() => name.to_owned(),
        _ => extract_name_from_path(&path),
    };
    FileDetail {
        name,
        id,
        path: path.to_owned(),
        exist: true,
        icon,
        version,
    }
}
