use super::models::CursorPosition;
use crate::storage::FILE_SHARE_READ;
use crate::util::{Timestamp, d_as_ms, now_day};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::prelude::OpenOptionsExt;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

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
    /// None in read-only mode; write functions must not be called then.
    file: Mutex<Option<File>>,
    /// Initial data recording date from `UNIX_EPOCH`.
    base_day: IndexUnit,
    record_index_vec: Mutex<Vec<IndexUnit>>,
}

fn get_state<'a>() -> &'a State {
    STATE.get().unwrap()
}

pub fn init(data_dir: impl AsRef<Path>) {
    init_inner(data_dir, false);
}

/// Initialize index storage in read-only mode.
/// A missing index.bin is treated as empty data.
pub fn init_readonly(data_dir: impl AsRef<Path>) {
    init_inner(data_dir, true);
}

fn init_inner(data_dir: impl AsRef<Path>, read_only: bool) {
    let path = data_dir.as_ref().join("index.bin");
    let mut file = if read_only {
        match OpenOptions::new().read(true).open(&path) {
            Ok(file) => file,
            Err(_) => {
                STATE
                    .set(State {
                        file: Mutex::new(None),
                        base_day: 0,
                        record_index_vec: Mutex::new(Vec::new()),
                    })
                    .unwrap();
                return;
            }
        }
    } else {
        OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&path)
            .expect("open index.bin failed.")
    };

    let mut index = read_index(&mut file);
    let base_day = if index.is_empty() {
        if read_only {
            0
        } else {
            let today = now_day() as IndexUnit;
            file.write(&today.to_le_bytes()).unwrap();
            index.push(0);
            today
        }
    } else {
        let ret = index[0];
        index[0] = 0;
        ret
    };
    STATE
        .set(State {
            file: Mutex::new((!read_only).then_some(file)),
            base_day,
            record_index_vec: Mutex::new(index),
        })
        .unwrap();
}

pub(crate) fn query_index(day: IndexUnit) -> CursorPosition {
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
    let file = file
        .as_mut()
        .expect("Cannot write to read-only storage.");
    index.push(value);
    file.write(&value.to_le_bytes()).unwrap();
}

fn last_day() -> IndexUnit {
    get_state().base_day + get_state().record_index_vec.lock().unwrap().len() as IndexUnit - 1
}

pub fn start_day() -> IndexUnit {
    get_state().base_day
}

/// Read index units from the file, tolerating a partially written trailing unit
/// when another process (the GUI) is appending concurrently.
fn read_index(file: &mut File) -> Vec<IndexUnit> {
    let mut ret = Vec::new();
    loop {
        let mut buf: IndexUnitByte = IndexUnitByte::default();
        let mut filled = 0;
        while filled < buf.len() {
            match file.read(&mut buf[filled..]) {
                Ok(0) => break,
                Ok(n) => filled += n,
                // Tolerate transient read errors during a concurrent append.
                Err(_) => return ret,
            }
        }
        if filled == buf.len() {
            ret.push(IndexUnit::from_le_bytes(buf));
        } else {
            break;
        }
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
