use std::cmp::max;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::Mutex;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use super::now_day;
use super::r#type::{Day, IndexUnit, IndexUnitByte, Millisecond};

pub enum IndexValue {
    Before,
    In(IndexUnit),
    After,
}

impl IndexValue {
    pub fn offset(self, value: i64) -> IndexValue {
        match self {
            IndexValue::Before => IndexValue::Before,
            IndexValue::After => IndexValue::After,
            IndexValue::In(v) => IndexValue::In(max(0, v + value)),
        }
    }
}

/// 2.85kB one year.
///
/// The first 8 bytes in file means the first day from `UNIX_EPOCH`.
/// After which, every 8 bytes represent the starting index of the corresponding day in record.bin.
/// Each value is the record file index of specific day.
pub struct FileIndex {
    file: Mutex<File>,
    base_day: Day,
    record_index_vec: Mutex<Vec<IndexUnit>>,
}

impl FileIndex {
    /// Create index.bin if not exist, and initialize it if empty.
    pub fn new(data_dir: &PathBuf) -> FileIndex {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ.0)
            .open(data_dir.join("index.bin"))
            .expect("open index.bin failed.");
        let mut index = Self::read_index(&mut file);
        let base_day = if index.is_empty() {
            let today = now_day() as IndexUnit;
            file.write(&today.to_le_bytes()).unwrap();
            index.push(0);
            today
        } else {
            let ret = index[0];
            index[0] = 0;
            ret
        };
        Self {
            file: Mutex::new(file),
            base_day,
            record_index_vec: Mutex::new(index),
        }
    }

    pub fn query_index(&self, day: Day) -> IndexValue {
        let index = self.record_index_vec.lock().unwrap();
        let size = index.len();
        match day - self.base_day {
            n if n < 0 => IndexValue::Before,
            n if n >= size as i64 => IndexValue::After,
            n => IndexValue::In(index[n as usize]),
        }
    }

    /// If the record start time is later than the last day, write the index to the file.
    pub fn update_index(&self, value: Millisecond, index: IndexUnit) {
        let day = value / (24 * 60 * 60 * 1000);
        let last_day = self.last_day();
        if day <= last_day {
            return;
        }
        for _ in last_day..day {
            self.write_index(index);
        }
    }

    /// Write the index to the file. The index is the starting index of the corresponding day in record.bin.
    fn write_index(&self, value: IndexUnit) {
        let mut index = self.record_index_vec.lock().unwrap();
        let mut file = self.file.lock().unwrap();
        index.push(value);
        file.write(&value.to_le_bytes()).unwrap();
    }


    fn last_day(&self) -> Day {
        self.base_day + self.record_index_vec.lock().unwrap().len() as i64 - 1
    }

    fn read_index(file: &mut File) -> Vec<IndexUnit> {
        let mut buf: IndexUnitByte = IndexUnitByte::default();
        let mut ret = Vec::new();
        while file.read(&mut buf).unwrap() != 0 {
            ret.push(IndexUnit::from_le_bytes(buf));
        }
        ret
    }
}
