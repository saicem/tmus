use std::cmp::min;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::mem::size_of;
use std::ops::Add;
use std::os::windows::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::Mutex;

use log::debug;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;
use crate::engine::data::{CursorPosition, ReadDirection};
use crate::engine::focus_record::RecordByte;

const RECORD_SIZE: usize = size_of::<RecordByte>();

pub struct FileRecord {
    file: Mutex<File>,
}

impl FileRecord {
    pub fn new(data_dir: &PathBuf) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ.0)
            .open(data_dir.join("record.bin"))
            .expect("open record.bin failed.");
        Self {
            file: Mutex::new(file),
        }
    }

    pub fn write(&self, record: RecordByte) -> u64 {
        let mut file = self.file.lock().unwrap();
        file.write(&record).unwrap();
        file.flush().unwrap();
        debug!("write record:{:?}", record);
        file.seek(SeekFrom::End(0)).unwrap() / RECORD_SIZE as u64
    }

    pub fn read_by_cursor(&self, cursor: CursorPosition, quantity: u64, direction: ReadDirection) -> (Vec<RecordByte>, CursorPosition) {
        if quantity == 0 {
            return (vec![], cursor);
        }
        let mut file = self.file.lock().unwrap();
        let (start, end) = compute_read_range(&mut file, cursor, quantity, direction);
        debug!("Read by cursor, start:{}, end:{}, count:{}", start, end, quantity);
        let mut ret = read(&mut file, start, end);
        if direction == ReadDirection::Backward {
            ret.reverse();
        }
        (ret, match direction {
            ReadDirection::Forward => CursorPosition::Middle(end),
            ReadDirection::Backward => start.checked_sub(1).map_or(CursorPosition::Start, |x| CursorPosition::Middle(x)),
        })
    }


    /// Read all record from start to end, end is not included.
    ///
    /// `read(start, end)` is equivalent to `read_by_cursor(CursorPosition::Middle(start), end - start, ReadDirection::Forward)`
    pub fn read(&self, start: u64, end: u64) -> Vec<RecordByte> {
        let mut file = self.file.lock().unwrap();
        read(&mut file, start, end)
    }

    pub fn read_to_end(&self, start: u64) -> Vec<RecordByte> {
        let mut file = self.file.lock().unwrap();
        let total_count = file.seek(SeekFrom::End(0)).unwrap() / RECORD_SIZE as u64;
        read(&mut file, start, total_count)
    }
}

fn compute_read_range(file: &mut File, cursor_position: CursorPosition, quantity: u64, direction: ReadDirection) -> (u64, u64) {
    let total_count = file.seek(SeekFrom::End(0)).unwrap() / RECORD_SIZE as u64;
    let cur = match cursor_position {
        CursorPosition::Start => { 0 }
        CursorPosition::End => { total_count }
        CursorPosition::Middle(cur) => { cur }
    };
    match direction {
        ReadDirection::Forward => {
            let end = min(cur.add(quantity), total_count);
            (cur, end)
        }
        ReadDirection::Backward => {
            let start = cur.add(1).saturating_sub(quantity);
            (start, cur.add(1))
        }
    }
}

pub fn read(file: &mut File, ge: u64, lt: u64) -> Vec<RecordByte> {
    if lt <= ge {
        return vec![];
    }
    let mut buf: RecordByte = RecordByte::default();
    let mut ret = Vec::with_capacity((lt - ge) as usize);
    file.seek(SeekFrom::Start(ge * RECORD_SIZE as u64))
        .unwrap();
    for _ in ge..lt {
        let n = file.read(&mut buf).unwrap();
        if n != RECORD_SIZE {
            break;
        }
        ret.push(buf);
    }
    ret
}
