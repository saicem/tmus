use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::mem::size_of;
use std::os::windows::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::Mutex;

use log::debug;
use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use super::focus_record::RecordByte;

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
        return file.seek(SeekFrom::End(0)).unwrap() / RECORD_SIZE as u64;
    }

    /// End is not include.
    pub fn read(&self, start: u64, end: u64) -> Vec<RecordByte> {
        let mut buf: RecordByte = RecordByte::default();
        let mut ret = Vec::new();
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(start * RECORD_SIZE as u64))
            .unwrap();
        let times = end - start;
        for _ in 0..times {
            let n = file.read(&mut buf).unwrap();
            if n != RECORD_SIZE {
                break;
            }
            ret.push(buf);
        }
        ret
    }

    pub fn read_to_end(&self, start: u64) -> Vec<RecordByte> {
        let mut buf: RecordByte = RecordByte::default();
        let mut ret = Vec::new();
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(start * RECORD_SIZE as u64))
            .unwrap();
        loop {
            let n = file.read(&mut buf).unwrap();
            if n != RECORD_SIZE {
                break;
            }
            ret.push(buf);
        }
        ret
    }

    pub fn read_reverse(&self, cursor: Option<u64>, count: u64) -> (Vec<RecordByte>, u64) {
        if count == 0 {
            return (vec![], 0);
        }
        let mut buf: RecordByte = RecordByte::default();
        let mut file = self.file.lock().unwrap();
        let start = cursor
            .unwrap_or_else(|| file.seek(SeekFrom::End(0)).unwrap() / RECORD_SIZE as u64)
            .saturating_sub(count);
        debug!("read_reverse start:{}", start);
        file.seek(SeekFrom::Start(start * RECORD_SIZE as u64))
            .unwrap();
        let mut ret = vec![];
        for _ in 0..count {
            let n = file.read(&mut buf).unwrap();
            if n != RECORD_SIZE {
                break;
            }
            ret.push(buf)
        }
        ret.reverse();
        return (ret, start);
    }
}
