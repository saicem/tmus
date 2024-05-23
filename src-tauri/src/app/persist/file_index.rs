use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use once_cell::sync::OnceCell;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use crate::app::data::TmusTick;
use crate::upk;

/// 2.85kB one year.
///
/// The first 8 bytes in file means the first day from `UNIX_EPOCH`.
/// After which, every 8 bytes represent the starting index of the corresponding day in record.bin.use std::cell::OnceCell;
static FILE: OnceLock<Mutex<File>> = OnceLock::new();
static EPOCH_DAY: OnceCell<u64> = OnceCell::new();
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
        let today = TmusTick::now().day();
        file.write(&today.to_le_bytes()).unwrap();
        today
    } else {
        index.pop().unwrap()
    };
    FILE.set(Mutex::new(file)).unwrap();
    EPOCH_DAY.set(epoch_day).unwrap();
    INDEX.set(Mutex::new(index)).unwrap();
}

pub fn write_index(value: u64) {
    upk!(INDEX).push(value);
    upk!(FILE).write(&value.to_le_bytes()).unwrap();
}

/// Query index range based on date.
///
/// If the return value is u64::MAX, it indicates the end of the record file.
pub fn query_index(start_day: u64, end_day: u64) -> (u64, u64) {
    let epoch_day = EPOCH_DAY.get().unwrap().clone();
    let index = upk!(INDEX);
    // the start index of given day
    let calculate_index = |day: u64| -> u64 {
        let dif = day.saturating_sub(epoch_day) as usize;
        if dif <= 0 {
            0
        } else if dif >= index.len() {
            u64::MAX
        } else {
            index.get(dif).unwrap().clone()
        }
    };
    let start_index = calculate_index(start_day);
    let end_index = calculate_index(end_day);
    (start_index, end_index)
}

fn read_index(file: &mut File) -> Vec<u64> {
    let mut buf: [u8; 8] = [0; 8];
    let mut ret = Vec::new();
    while file.read(&mut buf).unwrap() != 0 {
        ret.push(u64::from_le_bytes(buf));
    }
    ret
}
