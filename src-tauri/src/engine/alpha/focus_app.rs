use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use super::data::AppId;

static STATE: OnceLock<State> = OnceLock::new();

#[derive(Debug)]
pub struct State {
    file: Mutex<File>,
    path_id_map: Mutex<HashMap<String, AppId>>,
    id_path_map: Mutex<Vec<String>>,
}

fn get_state<'a>() -> &'a State {
    STATE.get().unwrap()
}

pub fn init(data_dir: &PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .share_mode(FILE_SHARE_READ.0)
        .open(data_dir.join("app.txt"))
        .expect("open app.txt failed.");
    let id_path_map = read_apps(&mut file);
    let mut app_count = 0;
    let mut path_id_map = HashMap::new();
    for app in &id_path_map {
        path_id_map.insert(app.clone(), app_count);
        app_count += 1;
    }
    STATE
        .set(State {
            file: Mutex::new(file),
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
        .expect(&format!("id: {} not in app map.", id))
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
    let state = get_state();
    let mut id_name_map = state.id_path_map.lock().unwrap();
    let mut name_id_map = state.path_id_map.lock().unwrap();
    let mut file = state.file.lock().unwrap();

    let app_id = id_name_map.len() as AppId;
    name_id_map.insert(name.to_string(), app_id);
    id_name_map.push(name.to_string());
    file.write(format!("{}\n", name).as_bytes())
        .expect("can't write to app.txt");
    file.flush().expect("can't flush app.txt");
    app_id
}

fn read_apps(file: &mut File) -> Vec<String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}
