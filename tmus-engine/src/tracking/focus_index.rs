use super::models::CursorPosition;
use crate::util::{Timestamp, d_as_ms, now_day};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

type IndexUnitByte = [u8; 8];
type IndexUnit = u64;

static STATE: OnceLock<State> = OnceLock::new();

/// 2.85kB one year.
///
/// The first 8 bytes in file means the first day from `UNIX_EPOCH`.
/// After which, every 8 bytes represent the starting index of the corresponding day in record.bin.
/// Each value is the record file index of specific day.
#[derive(Debug)]
struct State {
    file: Mutex<File>,
    /// Initial data recording date from `UNIX_EPOCH`.
    base_day: IndexUnit,
    record_index_vec: Mutex<Vec<IndexUnit>>,
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
        .open(data_dir.join("index.bin"))
        .expect("open index.bin failed.");
    let mut index = read_index(&mut file);
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
    STATE
        .set(State {
            file: Mutex::new(file),
            base_day,
            record_index_vec: Mutex::new(index),
        })
        .unwrap();
}

pub fn query_index(day: IndexUnit) -> CursorPosition {
    let state = get_state();
    let base_day = state.base_day;
    let size = state.record_index_vec.lock().unwrap().len();
    day.checked_sub(base_day)
        .map(|n| {
            if n >= size as IndexUnit {
                CursorPosition::End
            } else {
                CursorPosition::Middle(state.record_index_vec.lock().unwrap()[n as usize] as usize)
            }
        })
        .unwrap_or(CursorPosition::Start)
}

/// If the record start time is later than the last day, write the index to the file.
pub fn update_index(day: IndexUnit, index: IndexUnit) {
    let last_day = last_day();
    if day <= last_day {
        return;
    }
    let state = get_state();
    for _ in last_day..day {
        write_index(index, state);
    }
}

/// Write the index to the file. The index is the starting index of the corresponding day in record.bin.
fn write_index(value: IndexUnit, state: &State) {
    let mut index = state.record_index_vec.lock().unwrap();
    let mut file = state.file.lock().unwrap();
    index.push(value);
    file.write(&value.to_le_bytes()).unwrap();
}

fn last_day() -> IndexUnit {
    get_state().base_day + get_state().record_index_vec.lock().unwrap().len() as IndexUnit - 1
}

pub fn start_day() -> IndexUnit {
    get_state().base_day
}

fn read_index(file: &mut File) -> Vec<IndexUnit> {
    let mut buf: IndexUnitByte = IndexUnitByte::default();
    let mut ret = Vec::new();
    while file.read(&mut buf).unwrap() != 0 {
        ret.push(IndexUnit::from_le_bytes(buf));
    }
    ret
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileIndexRecord {
    pub date_time: Timestamp,
    pub start_index: IndexUnit,
}

pub fn all_record() -> Vec<FileIndexRecord> {
    let (base_day, index_vec) = {
        let state = get_state();
        (
            state.base_day,
            state.record_index_vec.lock().unwrap().clone(),
        )
    };
    index_vec
        .iter()
        .enumerate()
        .map(|(i, &start_index)| FileIndexRecord {
            date_time: d_as_ms((base_day + i as IndexUnit) as i64),
            start_index,
        })
        .collect()
}
