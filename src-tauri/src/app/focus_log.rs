use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    os::windows::fs::OpenOptionsExt,
    path::PathBuf,
};


/**
 * app_id: ..16
 * focus_at: 16..40
 * duration: 40..
 */
pub struct FocusRecord {
    data: u64,
}

impl FocusRecord {
    pub fn new(app_id: u64, focus_at: u64, duration: u64) -> Self {
        FocusRecord {
            data: app_id | focus_at << 16 | duration << 40,
        }
    }

    pub fn app_id(&self) -> u64 {
        self.data & 0xffff
    }

    pub fn focus_at(&self) -> u64 {
        (self.data >> 16) & 0xffffff
    }

    pub fn duration(&self) -> u64 {
        (self.data >> 40) & 0xffffff
    }
}

pub struct FocusLogger {
    app_file: File,
    record_file: File,
    index_file: File,
}

impl FocusLogger {
    pub fn new(data_dir: &PathBuf) -> Self {
        let app_path = data_dir.join("app.txt");
        let record_path = data_dir.join("record.bin");
        let index_path = data_dir.join("index.bin");
        println!(
            "{:?}",
            data_dir.clone(),
        );
        let app_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(app_path)
            .expect("open app.txt failed.");
        let record_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(record_path)
            .expect("open record.bin failed.");
        let index_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(index_path)
            .expect("open index.bin failed.");
        Self {
            app_file,
            record_file,
            index_file,
        }
    }

    pub fn read_apps(&mut self) -> Vec<String> {
        let mut buf = String::new();
        self.app_file.read_to_string(&mut buf).unwrap();
        let mut apps = buf
            .split('\n')
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        if apps.last().is_some_and(|x| x.is_empty()) {
            apps.pop();
        }
        return apps;
    }

    pub fn write_app(&mut self, app: &str) {
        self.app_file
            .write(app.as_bytes())
            .expect("can't write to app.txt");
        self.app_file
            .write("\n".as_bytes())
            .expect("can't write to app.txt");
    }

    pub fn record_position(&mut self) -> u64 {
        return self.record_file.stream_position().unwrap();
    }

    pub fn write_record(&mut self, record: FocusRecord) {
        self.record_file.write(&record.data.to_le_bytes()).unwrap();
    }

    pub fn read_record(&mut self) -> Vec<FocusRecord> {
        let mut buf: [u8; 8] = [0; 8];
        let mut ret = Vec::new();
        loop {
            let n = self.record_file.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            if n != 8 {
                panic!("record.bin corrupted.");
            }
            ret.push(FocusRecord { data: u64::from_le_bytes(buf) });
        }
        ret
    }

    pub fn write_index(&mut self, value: u64) {
        self.index_file.write(&value.to_le_bytes()).unwrap();
    }

    pub fn read_index(&mut self) -> Vec<u64> {
        let mut buf: [u8; 8] = [0; 8];
        let mut ret = Vec::new();
        loop {
            let n = self.index_file.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            if n != 8 {
                panic!("index.bin corrupted.")
            }
            ret.push(u64::from_le_bytes(buf));
        }
        ret
    }
}

const FILE_SHARE_READ: u32 = 1;

pub fn write_focus_record(record: FocusRecord) {
    let mut test_file = OpenOptions::new()
        .write(true)
        .share_mode(FILE_SHARE_READ)
        .open("test.bin")
        .unwrap();
    test_file.write(&record.data.to_le_bytes()).unwrap();
}

pub fn read_focus_record() {
    let mut fil = File::open("test.bin").unwrap();
    let mut buf = [0; 8];
    while fil.read_exact(&mut buf).is_ok() {
        println!("Hello, world!{}", u64::from_le_bytes(buf));
    }
}
