use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::Mutex;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use super::r#type::AppId;

pub struct FileApp {
    file: Mutex<File>,
    path_id_map: Mutex<HashMap<String, AppId>>,
    id_path_map: Mutex<Vec<String>>,
}

impl FileApp {
    pub fn new(data_dir: &PathBuf) -> Self {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ.0)
            .open(data_dir.join("app.txt"))
            .expect("open app.txt failed.");
        let id_path_map = Self::read_apps(&mut file);
        let mut app_count = 0;
        let mut path_id_map = HashMap::new();
        for app in &id_path_map {
            path_id_map.insert(app.clone(), app_count);
            app_count += 1;
        }
        Self {
            file: Mutex::new(file),
            path_id_map: Mutex::new(path_id_map),
            id_path_map: Mutex::new(id_path_map),
        }
    }

    pub fn get_path_by_id(&self, id: AppId) -> String {
        self.id_path_map
            .lock()
            .unwrap()
            .get(id)
            .expect(&format!("id: {} not in app map.", id))
            .to_string()
    }

    pub fn get_id_by_path(&self, name: &str) -> AppId {
        let get = || {
            self.path_id_map
                .lock()
                .unwrap()
                .get(name)
                .map(|x| x.to_owned())
        };
        match get() {
            None => self.write_app(name),
            Some(x) => x,
        }
    }

    /// Returns the app id which was written.
    fn write_app(&self, name: &str) -> AppId {
        let mut id_name_map = self.id_path_map.lock().unwrap();
        let mut name_id_map = self.path_id_map.lock().unwrap();
        let mut file = self.file.lock().unwrap();

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
}
