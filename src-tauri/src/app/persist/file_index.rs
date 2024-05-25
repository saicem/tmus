use std::cmp::{max, min};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use once_cell::sync::OnceCell;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use crate::app::data::Tick;
use crate::upk;

/// 2.85kB one year.
///
/// The first 8 bytes in file means the first day from `UNIX_EPOCH`.
/// After which, every 8 bytes represent the starting index of the corresponding day in record.bin.use std::cell::OnceCell;
static FILE: OnceLock<Mutex<File>> = OnceLock::new();
static EPOCH_DAY: OnceCell<u64> = OnceCell::new();
/// Each value is the record file index of specific day, the index 0 in vec means the epoch day.
static INDEX: OnceLock<Mutex<Vec<u64>>> = OnceLock::new();

/// Create index.bin if not exist, and initialize it if empty.
pub fn init(data_dir: &PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .share_mode(FILE_SHARE_READ.0)
        .open(data_dir.join("index.bin"))
        .expect("open index.bin failed.");
    let mut index = read_index(&mut file);
    let epoch_day = if index.is_empty() {
        let today = Tick::now().day();
        file.write(&today.to_le_bytes()).unwrap();
        index.push(0);
        today
    } else {
        let ret = index[0];
        index[0] = 0;
        ret
    };
    FILE.set(Mutex::new(file)).unwrap();
    EPOCH_DAY.set(epoch_day).unwrap();
    INDEX.set(Mutex::new(index)).unwrap();
}

/// Query index between two date, both start day and end day are include.
///
/// If the return value is u64::MAX, it indicates the end of the record file.
pub fn query_index(start_day: u64, end_day: u64) -> Vec<u64> {
    let epoch_day = EPOCH_DAY.get().unwrap().clone();
    let index = upk!(INDEX);
    let last_day = epoch_day + index.len() as u64 - 1;
    if end_day <= epoch_day {
        return vec![0; (end_day - start_day + 1) as usize];
    }
    if start_day > last_day {
        return vec![u64::MAX; (end_day - start_day + 1) as usize];
    }
    let mut ret: Vec<u64> = Vec::new();
    if start_day < epoch_day {
        ret.extend(vec![0_u64; (epoch_day - start_day) as usize]);
    }
    let l = (max(start_day, epoch_day) - epoch_day) as usize;
    let r = ((min(end_day, last_day)) - epoch_day) as usize;
    ret.extend(index[l..=r].to_vec());
    if end_day > last_day {
        ret.extend(vec![u64::MAX; (end_day - last_day) as usize]);
    }
    ret
}

pub fn write_index(value: u64) {
    upk!(INDEX).push(value);
    upk!(FILE).write(&value.to_le_bytes()).unwrap();
}

fn read_index(file: &mut File) -> Vec<u64> {
    let mut buf: [u8; 8] = [0; 8];
    let mut ret = Vec::new();
    while file.read(&mut buf).unwrap() != 0 {
        ret.push(u64::from_le_bytes(buf));
    }
    ret
}
