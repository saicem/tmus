use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

/// App id start from 0.
#[derive(Debug)]
pub struct AppTxtFile {
    file: File,
    name_id_map: HashMap<String, u64>,
    apps: Vec<String>,
    app_count: u64,
}

impl AppTxtFile {
    pub fn new(data_dir: &PathBuf) -> AppTxtFile {
        let app_path = data_dir.join("app.txt");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ.0)
            .open(app_path)
            .expect("open app.txt failed.");
        let apps = read_apps(&mut file);
        let mut app_count = 0;
        let mut name_id_map = HashMap::new();
        for app in &apps {
            name_id_map.insert(app.clone(), app_count);
            app_count += 1;
        }
        Self {
            file,
            name_id_map,
            apps,
            app_count,
        }
    }

    pub fn get_name_by_id(&mut self, id: u64) -> String {
        self.apps
            .get(id as usize)
            .expect(&format!("id: {} not in app map.", id))
            .to_string()
    }

    pub fn get_id_by_name(&mut self, name: &str) -> u64 {
        match self.name_id_map.get(name) {
            None => self.write_app(name),
            Some(x) => x.clone(),
        }
    }

    /// Returns app_id which was written.
    pub fn write_app(&mut self, name: &str) -> u64 {
        let app_id = self.app_count;
        self.app_count += 1;
        self.name_id_map.insert(name.to_string(), self.app_count);
        self.apps.push(name.to_string());
        self.file
            .write(format!("{}\n", name).as_bytes())
            .expect("can't write to app.txt");
        app_id
    }

    pub fn read_apps(&mut self) -> Vec<String> {
        read_apps(&mut self.file)
    }
}

fn read_apps(file: &mut File) -> Vec<String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}
