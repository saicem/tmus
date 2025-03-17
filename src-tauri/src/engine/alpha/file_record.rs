use std::cmp::min;
use std::fs::OpenOptions;
use std::mem::size_of;
use std::path::{Path, PathBuf};

use crate::engine::data::focus_record::RecordByte;
use log;
use memmap2::MmapMut;

const RECORD_SIZE: usize = size_of::<RecordByte>();

pub struct FileRecord {
    file_path: PathBuf,
    mmap: MmapMut,
    len: usize,
    size: usize,
}

/// Find first all zero 8 bytes, and return the index of it.
fn find_really_len(arr: &[u8]) -> usize {
    for (index, chunk) in arr.chunks(8).enumerate() {
        if chunk.iter().all(|byte| *byte == 0) {
            return index * 8;
        }
    }
    arr.len()
}

/// The size use for mmap expand every time.
const EXPAND_SIZE: usize = 4 * 1024;

impl FileRecord {
    pub fn new(data_dir: &PathBuf) -> Self {
        let file_path = data_dir.join("record.bin");
        let mmap = Self::map_file(&file_path, None);
        let size = mmap.len() / 8 * 8;
        let search_start = size.saturating_sub(4 * 1024);
        let len = search_start + find_really_len(&mmap[search_start..size]);
        Self {
            file_path,
            mmap,
            len,
            size,
        }
    }

    pub fn can_append(&self) -> bool {
        self.size > self.len
    }

    pub fn expand_size(&mut self) {
        let new_size = self.size + EXPAND_SIZE;
        self.mmap = Self::map_file(&self.file_path, Some(new_size));
        self.size = new_size;
    }

    fn map_file<T: AsRef<Path>>(file_path: T, size: Option<usize>) -> MmapMut {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)
            .expect("open record.bin failed.");
        if let Some(size) = size {
            file.set_len(size as u64).expect("Resize file failed.");
        }
        unsafe { MmapMut::map_mut(&file).expect("Error mapping record.bin file") }
    }

    pub fn write(&mut self, record: RecordByte) -> u64 {
        if !self.can_append() {
            self.expand_size();
        }
        self.mmap[self.len..self.len + RECORD_SIZE].copy_from_slice(&record);
        self.len += 8;
        log::debug!(
            "write record:{}",
            record
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>()
        );
        (self.len / RECORD_SIZE) as u64
    }

    pub fn read(&self, start: usize, end: usize) -> Vec<RecordByte> {
        let start = start * RECORD_SIZE;
        let end = min(end * RECORD_SIZE, self.len);
        if start >= end {
            return vec![];
        }
        self.mmap[start..end]
            .chunks(RECORD_SIZE)
            .map(|chunk| chunk.try_into().unwrap())
            .collect()
    }

    pub fn read_to_end(&self, start: usize) -> Vec<RecordByte> {
        self.read(start, self.len)
    }
}
