use super::models::AppId;
use crate::storage::FILE_SHARE_READ;
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
