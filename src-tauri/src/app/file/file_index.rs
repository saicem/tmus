use crate::app::file::time::today;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

/// 2.85kB one year.
///
/// The first 8 bytes in file means the first day from `UNIX_EPOCH`.
/// After which, every 8 bytes represent the starting index of the corresponding day in record.bin.
pub struct IndexBinFile {
    file: File,
    start_day: u64,
    index: Vec<u64>,
}

impl IndexBinFile {
    /// Create index.bin if not exist, and initialize it if empty.
    pub fn new(data_dir: &PathBuf) -> IndexBinFile {
        let index_path = data_dir.join("index.bin");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ.0)
            .open(index_path)
            .expect("open index.bin failed.");
        let mut index = read_index(&mut file);
        let start_day = if index.is_empty() {
            let today = today();
            file.write(&today.to_le_bytes()).unwrap();
            today
        } else {
            index.pop().unwrap()
        };
        Self {
            file,
            start_day,
            index,
        }
    }

    pub fn write_index(&mut self, value: u64) {
        self.index.push(value);
        self.file.write(&value.to_le_bytes()).unwrap();
    }
}

fn read_index(file: &mut File) -> Vec<u64> {
    let mut buf: [u8; 8] = [0; 8];
    let mut ret = Vec::new();
    while file.read(&mut buf).unwrap() != 0 {
        ret.push(u64::from_le_bytes(buf));
    }
    ret
}
