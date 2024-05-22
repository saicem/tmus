use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::windows::fs::OpenOptionsExt;
use std::path::PathBuf;

use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;

use crate::app::data::focus_record::FocusRecord;

#[derive(Debug)]
pub struct RecordBinFile {
    file: File,
}

impl RecordBinFile {
    pub fn new(data_dir: &PathBuf) -> Self {
        let file = Self::init_file(data_dir);
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

    fn init_file(data_dir: &PathBuf) -> File {
        OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .share_mode(FILE_SHARE_READ.0)
            .open(data_dir.join("record.bin"))
            .expect("open record.bin failed.")
    }
}
