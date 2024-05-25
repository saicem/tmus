use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use crate::upk;

static FILE: OnceLock<Mutex<File>> = OnceLock::new();
static PATH_ID_MAP: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();
static ID_PATH_MAP: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

pub fn init(data_dir: &PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .share_mode(FILE_SHARE_READ.0)
        .open(data_dir.join("app.txt"))
        .expect("open app.txt failed.");
    let apps = read_apps(&mut file);
    let mut app_count = 0;
    let mut name_id_map = HashMap::new();
    for app in &apps {
        name_id_map.insert(app.clone(), app_count);
        app_count += 1;
    }
    FILE.set(Mutex::new(file)).unwrap();
    PATH_ID_MAP.set(Mutex::new(name_id_map)).unwrap();
    ID_PATH_MAP.set(Mutex::new(apps)).unwrap()
}

#[cfg(debug_assertions)]
pub fn get_path_by_id(id: usize) -> String {
    upk!(ID_PATH_MAP)
        .get(id)
        .map_or(format!("id: {} not in app map.", id), ToString::to_string)
}

#[cfg(not(debug_assertions))]
pub fn get_path_by_id(id: usize) -> String {
    upk!(ID_PATH_MAP)
        .get(id)
        .expect(&format!("id: {} not in app map.", id))
        .to_string()
}

pub fn get_id_by_path(name: &str) -> u64 {
    let get = || upk!(PATH_ID_MAP).get(name).map(|x| x.to_owned());
    match get() {
        None => write_app(name),
        Some(x) => x,
    }
}

/// Returns the app id which was written.
pub fn write_app(name: &str) -> u64 {
    let mut id_name_map = upk!(ID_PATH_MAP);
    let mut name_id_map = upk!(PATH_ID_MAP);
    let app_id = id_name_map.len() as u64;
    name_id_map.insert(name.to_string(), app_id);
    id_name_map.push(name.to_string());
    let mut file = upk!(FILE);
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
