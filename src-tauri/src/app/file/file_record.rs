use std::io::{Seek, SeekFrom};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::windows::fs::OpenOptionsExt,
    path::PathBuf,
};
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

#[derive(Debug)]
pub struct RecordBinFile {
    file: File,
}

impl RecordBinFile {
    pub fn new(data_dir: &PathBuf) -> Self {
        let record_path = data_dir.join("record.bin");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ.0)
            .open(record_path)
            .expect("open record.bin failed.");
        Self { file }
    }

    pub fn write_record(&mut self, record: FocusRecord) {
        self.file.write(&record.data.to_le_bytes()).unwrap();
    }

    pub fn read_record(&mut self, start: u64, end: u64) -> Vec<FocusRecord> {
        let mut buf: [u8; 8] = [0; 8];
        let mut ret = Vec::new();
        self.file.seek(SeekFrom::Start(start)).unwrap();
        let times = (end - start) / 8;
        for _ in 0..times {
            let n = self.file.read(&mut buf).unwrap();
            if n != 8 {
                break;
            }
            ret.push(FocusRecord {
                data: u64::from_le_bytes(buf),
            });
        }
        ret
    }

    pub fn read_to_end(&mut self, start: u64) -> Vec<FocusRecord> {
        let mut buf: [u8; 8] = [0; 8];
        let mut ret = Vec::new();
        self.file.seek(SeekFrom::Start(start)).unwrap();
        while self.file.read(&mut buf).unwrap() != 0 {
            ret.push(FocusRecord {
                data: u64::from_le_bytes(buf),
            });
        }
        ret
    }

    pub fn cur_position(&mut self) -> u64 {
        self.file.seek(SeekFrom::End(0)).unwrap()
    }
}

/// The meaning of 64bit as follows
/// - app_id: 0..16
/// - focus_at: 16..40
/// - duration: 40..64
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
