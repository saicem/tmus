use crate::app::constant::app_detail_cache_path;
use crate::util;
use crate::util::{dump_json, load_json, FileVersion};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tmus_engine::models::AppId;
use tmus_engine::storage::focus_app;
use tokio::sync::{Mutex, MutexGuard};
use windows_icons::get_icon_base64_by_path;

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

pub async fn get_app_detail_cache<'a>() -> MutexGuard<'a, HashMap<usize, FileDetail>> {
    static APP_DETAIL_CACHE: OnceLock<Mutex<HashMap<usize, FileDetail>>> = OnceLock::new();
    APP_DETAIL_CACHE
        .get_or_init(|| {
            let details: Vec<FileDetail> = load_json(app_detail_cache_path());
            Mutex::new(
                details
                    .into_iter()
                    .map(|detail| (detail.id, detail))
                    .collect(),
            )
        })
        .lock()
        .await
}

pub async fn update_app_detail_cache(
    app_detail_map: &mut HashMap<usize, FileDetail>,
    details: Vec<FileDetail>,
) {
    for detail in details {
        app_detail_map.insert(detail.id, detail);
    }
    if let Err(e) = dump_json(
        &app_detail_map
            .values()
            .map(|x| x.to_owned())
            .collect::<Vec<FileDetail>>(),
        app_detail_cache_path(),
    ) {
        tracing::error!("Failed to save app detail cache: {}", e);
    }
}

#[tauri::command]
#[tracing::instrument]
pub async fn get_app_detail(id: usize) -> FileDetail {
    let path = focus_app::get_path_by_id(id);
    let mut detail = query_file_detail(id, &path);
    let mut app_detail_cache = get_app_detail_cache().await;
    // File has been deleted, use cache data only when it still matches the
    // current app.txt entry for this id.
    if !detail.exist {
        if let Some(cached_detail) = app_detail_cache.get_mut(&id) {
            if cached_detail.path == path {
                detail = cached_detail.clone();
                detail.exist = false;
            }
        }
    }
    update_app_detail_cache(&mut app_detail_cache, vec![detail.clone()]).await;
    detail
}

#[tauri::command]
#[tracing::instrument]
pub async fn get_all_app_detail() -> HashMap<AppId, FileDetail> {
    let app_vec = focus_app::get_all_app();
    let mut app_detail_cache = get_app_detail_cache().await;
    let mut cache_changed = false;

    // Drop stale cache entries whose path no longer matches app.txt (the file
    // can be rewritten, shifting ids), and entries for duplicate or merged app
    // ids so one logical app is listed only once.
    let canonical_id_map = focus_app::get_canonical_id_map();
    app_detail_cache.retain(|id, detail| {
        let valid = app_vec.get(*id).is_some_and(|path| path == &detail.path)
            && canonical_id_map.get(*id).copied().unwrap_or(*id) == *id;
        if !valid {
            cache_changed = true;
        }
        valid
    });

    for detail in app_detail_cache.values_mut() {
        detail.exist = Path::new(&detail.path).exists();
        // Store apps keep their real icon as manifest assets; refresh it so
        // upgrades never leave a generic or stale icon behind.
        if detail.exist && is_windows_apps_path(&detail.path) {
            if let Some(icon) = get_app_icon_base64(&detail.path) {
                if detail.icon.as_deref() != Some(icon.as_str()) {
                    detail.icon = Some(icon);
                    cache_changed = true;
                }
            }
        }
    }
    let not_exist_app_detail: Vec<FileDetail> = app_vec
        .iter()
        .enumerate()
        .filter(|(id, _)| !app_detail_cache.contains_key(id))
        .filter(|(id, _)| canonical_id_map.get(*id).copied().unwrap_or(*id) == *id)
        .map(|(id, path)| query_file_detail(id, path))
        .collect();
    if !not_exist_app_detail.is_empty() || cache_changed {
        update_app_detail_cache(&mut app_detail_cache, not_exist_app_detail).await;
    }
    app_detail_cache.clone()
}

fn query_file_detail(id: usize, path: &str) -> FileDetail {
    let extract_name_from_path = |path: &str| {
        Path::new(&path)
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .map_or_else(|| path.to_owned(), |file_stem| file_stem.to_owned())
    };
    if !Path::new(&path).exists() {
        return FileDetail {
            name: extract_name_from_path(path),
            id,
            path: path.to_owned(),
            exist: false,
            icon: None,
            version: None,
        };
    }
    let version = util::get_file_version(path);
    let icon = get_app_icon_base64(path);
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
        _ => extract_name_from_path(path),
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

/// Best-effort icon retrieval. The icon embedded in the executable is the
/// actual app icon (the one Explorer and the taskbar show), so try it first.
/// For WindowsApps (MSIX/Store) apps fall back to the manifest tile logo,
/// which is only used when the executable exposes no icon at all.
fn get_app_icon_base64(path: &str) -> Option<String> {
    let exe_icon = get_icon_base64_by_path(path)
        .ok()
        .map(|base64| format!("data:image/png;base64,{base64}"));
    if !is_windows_apps_path(path) {
        return exe_icon;
    }
    exe_icon.or_else(|| get_windows_apps_icon_base64(path))
}

fn is_windows_apps_path(path: &str) -> bool {
    path.to_lowercase().contains("\\windowsapps\\")
}

/// Locate AppxManifest.xml and load the package logo as a base64 data URL.
/// Monochrome glyph logos are composited onto the tile background color from
/// the manifest so they stay visible on any theme.
fn get_windows_apps_icon_base64(exe_path: &str) -> Option<String> {
    let package_root = Path::new(exe_path)
        .ancestors()
        .take(6)
        .find(|dir| dir.join("AppxManifest.xml").is_file())?;
    let manifest = fs::read_to_string(package_root.join("AppxManifest.xml")).ok()?;
    let background = parse_background_color(&manifest);
    let logo = extract_manifest_logo(&manifest)?;
    let logo_path = resolve_logo_path(package_root, &logo)?;
    let bytes = fs::read(logo_path).ok()?;
    let bytes = composite_icon_with_background(&bytes, background).unwrap_or(bytes);
    let base64 = STANDARD.encode(bytes);
    Some(format!("data:image/png;base64,{base64}"))
}

/// Extract the app logo from AppxManifest.xml: prefer the high-resolution
/// VisualElements square logos, falling back to the classic `<Logo>` element.
fn extract_manifest_logo(manifest: &str) -> Option<String> {
    for attribute in [
        "Square150x150Logo",
        "Square44x44Logo",
        "StoreLogo",
        "Square71x71Logo",
    ] {
        if let Some(value) = extract_xml_attribute(manifest, attribute) {
            return Some(value);
        }
    }
    const LOGO_START: &str = "<Logo>";
    if let Some(start) = manifest.find(LOGO_START) {
        let value_start = start + LOGO_START.len();
        if let Some(end) = manifest[value_start..].find("</Logo>") {
            let value = manifest[value_start..value_start + end].trim();
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
    }
    None
}

/// Parse the tile `BackgroundColor` attribute (`#RRGGBB` or `#AARRGGBB`).
fn parse_background_color(manifest: &str) -> Option<[u8; 4]> {
    let value = extract_xml_attribute(manifest, "BackgroundColor")?;
    let hex = value.trim().strip_prefix('#').unwrap_or(value.trim());
    let parsed = u32::from_str_radix(hex, 16).ok()?;
    Some(match hex.len() {
        6 => [
            ((parsed >> 16) & 0xFF) as u8,
            ((parsed >> 8) & 0xFF) as u8,
            (parsed & 0xFF) as u8,
            255,
        ],
        8 => [
            ((parsed >> 16) & 0xFF) as u8,
            ((parsed >> 8) & 0xFF) as u8,
            (parsed & 0xFF) as u8,
            ((parsed >> 24) & 0xFF) as u8,
        ],
        _ => return None,
    })
}

/// Fill in a solid background for monochrome glyph logos that ship with a
/// transparent canvas (e.g. white tile logos). Returns None to pass the
/// original bytes through unchanged: when no background color is known, the
/// icon is full-color, or it is already mostly opaque.
fn composite_icon_with_background(bytes: &[u8], background: Option<[u8; 4]>) -> Option<Vec<u8>> {
    let background = background?;
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    if width == 0 || height == 0 {
        return None;
    }

    let total = u64::from(width) * u64::from(height);
    let mut opaque = 0u64;
    let mut colored = false;
    for pixel in rgba.pixels() {
        if pixel[3] > 200 {
            opaque += 1;
            let max_channel = pixel[0].max(pixel[1]).max(pixel[2]);
            let min_channel = pixel[0].min(pixel[1]).min(pixel[2]);
            if max_channel - min_channel > 24 {
                colored = true;
            }
        }
    }
    // Skip icons that are full-color or already have a filled background.
    if colored || opaque * 100 >= total * 55 {
        return None;
    }

    let mut out = image::RgbaImage::from_pixel(
        width,
        height,
        image::Rgba([background[0], background[1], background[2], 255]),
    );
    image::imageops::overlay(&mut out, &rgba, 0, 0);
    let mut cursor = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(out)
        .write_to(&mut cursor, image::ImageFormat::Png)
        .ok()?;
    Some(cursor.into_inner())
}

fn extract_xml_attribute(xml: &str, name: &str) -> Option<String> {
    let search = format!("{name}=");
    let index = xml.find(&search)?;
    let rest = &xml[index + search.len()..];
    let mut chars = rest.chars();
    let quote = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let mut value = String::new();
    for c in chars {
        if c == quote {
            return Some(value);
        }
        value.push(c);
    }
    None
}

/// Resolve a manifest logo path to an existing file, falling back to the
/// scale/targetsize variants (e.g. `Square44x44Logo.scale-200.png`).
fn resolve_logo_path(package_root: &Path, logo: &str) -> Option<PathBuf> {
    let logo_path = package_root.join(logo.replace('/', "\\"));
    if logo_path.is_file() {
        return Some(logo_path);
    }
    let dir = logo_path.parent()?;
    let stem = logo_path.file_stem()?.to_str()?.to_owned();
    let extension = logo_path.extension()?.to_str()?.to_owned();
    let suffix = format!(".{extension}");
    let candidates: Vec<(PathBuf, u32)> = fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_str()?.to_owned();
            if !name.starts_with(&stem) || !name.ends_with(&suffix) {
                return None;
            }
            if name.contains("contrast-") || name.contains("altform-") {
                return None;
            }
            let scale = if name.contains("scale-200") {
                3
            } else if name.contains("scale-150") {
                2
            } else if name.contains("scale-100") {
                1
            } else {
                0
            };
            Some((entry.path(), scale))
        })
        .collect();
    candidates
        .into_iter()
        .max_by(|a, b| {
            let a_key = (a.1, a.0.metadata().ok().map_or(0, |m| m.len()));
            let b_key = (b.1, b.0.metadata().ok().map_or(0, |m| m.len()));
            a_key.cmp(&b_key)
        })
        .map(|(path, _)| path)
}

/// Lightweight id -> display name map read from the detail cache without
/// touching exe metadata or icons. Used by statistics where only names matter.
pub async fn get_app_name_map() -> HashMap<AppId, String> {
    get_app_detail_cache()
        .await
        .iter()
        .map(|(id, detail)| (*id, detail.name.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_background_color_supports_rgb_and_argb() {
        assert_eq!(
            parse_background_color(
                r##"<uap:VisualElements BackgroundColor="#3143FF"></uap:VisualElements>"##
            ),
            Some([0x31, 0x43, 0xFF, 255])
        );
        assert_eq!(
            parse_background_color(r##"BackgroundColor="#803143FF""##),
            Some([0x31, 0x43, 0xFF, 0x80])
        );
        assert_eq!(parse_background_color("<Logo>icon.png</Logo>"), None);
    }

    #[test]
    fn extract_manifest_logo_prefers_square_logo() {
        let manifest = concat!(
            "<Logo>assets/icon.png</Logo>",
            r#"<uap:VisualElements Square44x44Logo="assets/Square44x44Logo.png" "#,
            r#"Square150x150Logo="assets/Square150x150Logo.png" />"#,
        );
        assert_eq!(
            extract_manifest_logo(manifest).as_deref(),
            Some("assets/Square150x150Logo.png")
        );
    }
}
