use super::models::AppId;
use crate::storage::FILE_SHARE_READ;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use tracing::info;

static STATE: OnceLock<State> = OnceLock::new();

#[derive(Debug)]
pub struct State {
    /// None in read-only mode; write functions must not be called then.
    file: Mutex<Option<File>>,
    path_id_map: Mutex<HashMap<String, AppId>>,
    id_path_map: Mutex<Vec<String>>,
}

fn get_state<'a>() -> &'a State {
    STATE.get().unwrap()
}

pub fn init(data_dir: impl AsRef<Path>) {
    init_inner(data_dir, false);
}

/// Initialize app storage in read-only mode.
/// A missing app.txt is treated as empty data.
pub fn init_readonly(data_dir: impl AsRef<Path>) {
    init_inner(data_dir, true);
}

fn init_inner(data_dir: impl AsRef<Path>, read_only: bool) {
    let path = data_dir.as_ref().join("app.txt");
    let mut file = if read_only {
        match OpenOptions::new().read(true).open(&path) {
            Ok(file) => file,
            Err(_) => {
                STATE
                    .set(State {
                        file: Mutex::new(None),
                        path_id_map: Mutex::new(HashMap::new()),
                        id_path_map: Mutex::new(Vec::new()),
                    })
                    .unwrap();
                return;
            }
        }
    } else {
        OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&path)
            .expect("open app.txt failed.")
    };

    let id_path_map = read_apps(&mut file);
    let mut path_id_map = HashMap::new();
    for (app_count, app) in id_path_map.iter().enumerate() {
        path_id_map.insert(app.clone(), app_count);
    }
    STATE
        .set(State {
            file: Mutex::new((!read_only).then_some(file)),
            path_id_map: Mutex::new(path_id_map),
            id_path_map: Mutex::new(id_path_map),
        })
        .unwrap();
}

pub fn get_path_by_id(id: AppId) -> String {
    get_state()
        .id_path_map
        .lock()
        .unwrap()
        .get(id)
        .unwrap_or_else(|| panic!("id: {} not in app map.", id))
        .to_string()
}

pub fn get_id_by_path(name: &str) -> AppId {
    let id = {
        get_state()
            .path_id_map
            .lock()
            .unwrap()
            .get(name)
            .map(|x| x.to_owned())
    };
    match id {
        None => write_app(name),
        Some(x) => x,
    }
}

pub fn get_all_app() -> Vec<String> {
    get_state().id_path_map.lock().unwrap().to_owned()
}

/// WindowsApps (MSIX/Store) install paths use the Package Full Name layout:
/// `...\WindowsApps\<PackageName>_<Version>_<Architecture>__<PublisherId>\...`
/// e.g. `OpenAI.Codex_26.818.2441.0_x64__2p2nqsd0c76g0`.
///
/// Only `<Version>` and `<Architecture>` change on app updates; the stable
/// identity is the Package Family Name `<PackageName>_<PublisherId>`.
/// Normalizing to it keeps one logical app on one id across updates:
/// `...\WindowsApps\OpenAI.Codex_26.818.2441.0_x64__2p2nqsd0c76g0\app\ChatGPT.exe`
/// becomes `...\WindowsApps\OpenAI.Codex_2p2nqsd0c76g0\app\ChatGPT.exe`.
/// The in-package path is preserved so distinct apps shipped in one package
/// (e.g. two entry points of NanaZip) stay separate.
fn normalize_store_app_path(path: &str) -> String {
    static PACKAGE_SEGMENT: OnceLock<Regex> = OnceLock::new();
    let re = PACKAGE_SEGMENT.get_or_init(|| {
        Regex::new(
            r#"(?i)(\\WindowsApps\\[^\\]+?)_\d+(?:\.\d+)+_(?:x86|x64|arm|arm64|x86a64|neutral)__([0-9a-z]+)((?:\\[^\\]+)*)"#,
        )
        .expect("invalid WindowsApps package path regex")
    });
    re.replace_all(path, "${1}_${2}${3}").into_owned()
}

fn build_canonical_id_map(id_path_map: &[String]) -> Vec<AppId> {
    let mut canonical_by_path: HashMap<String, AppId> = HashMap::new();
    for (id, path) in id_path_map.iter().enumerate() {
        canonical_by_path.insert(normalize_store_app_path(path), id);
    }
    id_path_map
        .iter()
        .map(|path| canonical_by_path[&normalize_store_app_path(path)])
        .collect()
}

/// Build a map from every app id to the representative id of its
/// version-independent path group. Because the group is iterated in id order,
/// the newest id of each group wins, so all versions of a Store app share one
/// representative id for statistics.
pub fn get_canonical_id_map() -> Vec<AppId> {
    build_canonical_id_map(&get_state().id_path_map.lock().unwrap())
}

/// Resolve any app id to the representative id of its Store app group.
/// Ids outside the map (unreachable in practice) are returned unchanged.
pub fn get_canonical_id(id: AppId) -> AppId {
    get_canonical_id_map().get(id).copied().unwrap_or(id)
}

/// Returns the app id which was written.
fn write_app(name: &str) -> AppId {
    info!("New app found, write app: {}", name);
    let state = get_state();
    let mut id_name_map = state.id_path_map.lock().unwrap();
    let mut name_id_map = state.path_id_map.lock().unwrap();
    let mut file = state.file.lock().unwrap();
    let file = file.as_mut().expect("Cannot write to read-only storage.");

    let app_id = id_name_map.len() as AppId;
    name_id_map.insert(name.to_string(), app_id);
    id_name_map.push(name.to_string());
    file.write_all(format!("{}\n", name).as_bytes())
        .expect("can't write to app.txt");
    file.flush().expect("can't flush app.txt");
    app_id
}

/// Read app paths from the file. A trailing line without a newline is treated
/// as an in-progress append from the GUI and is ignored.
fn read_apps(file: &mut File) -> Vec<String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let complete = buf.strip_suffix('\n').unwrap_or("");
    complete
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_store_app_path() {
        assert_eq!(
            normalize_store_app_path(
                r"C:\Program Files\WindowsApps\OpenAI.Codex_26.818.2441.0_x64__2p2nqsd0c76g0\app\ChatGPT.exe"
            ),
            r"C:\Program Files\WindowsApps\OpenAI.Codex_2p2nqsd0c76g0\app\ChatGPT.exe"
        );
        // Version and architecture are stripped; publisher id and subpath stay.
        assert_eq!(
            normalize_store_app_path(
                r"C:\Program Files\WindowsApps\OpenAI.Codex_26.519.5221.0_x64__2p2nqsd0c76g0\app\Codex.exe"
            ),
            r"C:\Program Files\WindowsApps\OpenAI.Codex_2p2nqsd0c76g0\app\Codex.exe"
        );
        // Case-insensitive WindowsApps directory, neutral / x86a64 architectures.
        assert_eq!(
            normalize_store_app_path(
                r"C:\Program Files\windowsapps\Microsoft.WindowsTerminal_1.20.3181.0_neutral__8wekyb3d8bbwe\WindowsTerminal.exe"
            ),
            r"C:\Program Files\windowsapps\Microsoft.WindowsTerminal_8wekyb3d8bbwe\WindowsTerminal.exe"
        );
        assert_eq!(
            normalize_store_app_path(
                r"C:\Program Files\WindowsApps\OpenAI.Codex_26.818.2441.0_x86a64__2p2nqsd0c76g0\app\ChatGPT.exe"
            ),
            r"C:\Program Files\WindowsApps\OpenAI.Codex_2p2nqsd0c76g0\app\ChatGPT.exe"
        );
        // Version-looking digits inside the package name must not confuse the split.
        assert_eq!(
            normalize_store_app_path(
                r"C:\Program Files\WindowsApps\Microsoft.Windows.Photos_2025.11030.27002.0_x64__8wekyb3d8bbwe\Photos.exe"
            ),
            r"C:\Program Files\WindowsApps\Microsoft.Windows.Photos_8wekyb3d8bbwe\Photos.exe"
        );
        // Paths without the Store layout are untouched.
        assert_eq!(
            normalize_store_app_path(r"D:\Tools\Codex\Codex.exe"),
            r"D:\Tools\Codex\Codex.exe"
        );
        assert_eq!(
            normalize_store_app_path(r"C:\Windows\System32\notepad.exe"),
            r"C:\Windows\System32\notepad.exe"
        );
    }

    #[test]
    fn test_build_canonical_id_map() {
        let paths = vec![
            r"C:\Program Files\WindowsApps\OpenAI.Codex_26.818.2441.0_x64__2p2nqsd0c76g0\app\ChatGPT.exe".to_owned(),
            r"D:\Tools\Notepad\notepad.exe".to_owned(),
            r"C:\Program Files\WindowsApps\OpenAI.Codex_26.818.5229.0_x64__2p2nqsd0c76g0\app\ChatGPT.exe".to_owned(),
            r"C:\Program Files\WindowsApps\OpenAI.Codex_26.818.3698.0_x64__2p2nqsd0c76g0\app\ChatGPT.exe".to_owned(),
            r"C:\Program Files\WindowsApps\OpenAI.Codex_26.519.5221.0_x64__2p2nqsd0c76g0\app\Codex.exe".to_owned(),
        ];
        let map = build_canonical_id_map(&paths);
        // All versions of the same in-package executable resolve to the newest
        // id (3); the renamed entry point (Codex.exe) and other apps stay as-is.
        assert_eq!(map, vec![3, 1, 3, 3, 4]);
    }
}
