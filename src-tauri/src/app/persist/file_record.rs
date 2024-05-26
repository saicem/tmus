use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::windows::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use crate::app::data::FocusRecord;
use crate::upk;

static FILE: OnceLock<Mutex<File>> = OnceLock::new();

pub fn init(data_dir: &PathBuf) {
    let f = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .share_mode(FILE_SHARE_READ.0)
        .open(data_dir.join("record.bin"))
        .expect("open record.bin failed.");
    FILE.set(Mutex::new(f)).unwrap();
}

pub fn write_record(record: &FocusRecord) {
    let mut file = upk!(FILE);
    file.write(&record.raw.to_le_bytes()).unwrap();
    file.flush().unwrap();
}

pub fn read_record(start: u64, end: u64) -> Vec<FocusRecord> {
    if end == u64::MAX {
        return read_to_end(start);
    }
    let mut buf: [u8; 8] = [0; 8];
    let mut ret = Vec::new();
    let mut file = upk!(FILE);
    file.seek(SeekFrom::Start(start)).unwrap();
    let times = (end - start) / 8;
    for _ in 0..times {
        let n = file.read(&mut buf).unwrap();
        if n != 8 {
            break;
        }
        ret.push(FocusRecord {
            raw: u64::from_le_bytes(buf),
        });
    }
    ret
}

fn read_to_end(start: u64) -> Vec<FocusRecord> {
    let mut buf: [u8; 8] = [0; 8];
    let mut ret = Vec::new();
    let mut file = upk!(FILE);
    file.seek(SeekFrom::Start(start)).unwrap();
    while file.read(&mut buf).unwrap() != 0 {
        ret.push(FocusRecord {
            raw: u64::from_le_bytes(buf),
        });
    }
    ret
}

pub fn cur_position() -> u64 {
    upk!(FILE).seek(SeekFrom::End(0)).unwrap()
}
