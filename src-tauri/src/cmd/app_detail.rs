use crate::util;
use crate::util::FileVersion;
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

static APP_DETAIL_CACHE: OnceLock<Mutex<HashMap<usize, FileDetail>>> = OnceLock::new();

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
    APP_DETAIL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[tauri::command]
#[tracing::instrument]
pub async fn get_app_detail(id: usize) -> FileDetail {
    let path = focus_app::get_path_by_id(id);
    let detail = query_file_detail(id, &path);
    get_app_detail_cache()
        .lock()
        .await
        .insert(id, detail.clone());
    detail
}

#[tauri::command]
#[tracing::instrument]
pub async fn get_all_app_detail() -> Result<Vec<FileDetail>, String> {
    let app_vec = focus_app::get_all_app();
    let mut app_detail_cache = get_app_detail_cache().lock().await;
    Ok(app_vec
        .iter()
        .enumerate()
        .map(|(id, path)| {
            app_detail_cache
                .entry(id)
                .or_insert_with(|| query_file_detail(id, path))
                .clone()
        })
        .collect())
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
