use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::windows::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::Mutex;

use windows::Win32::Storage::FileSystem::FILE_SHARE_READ;


pub struct FileRecord<const N: usize> {
    file: Mutex<File>,
}

impl<const N: usize> FileRecord<N> {
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

    pub fn write(&self, record: [u8; N]) -> u64 {
        let mut file = self.file.lock().unwrap();
        file.write(&record).unwrap();
        file.flush().unwrap();
        return file.seek(SeekFrom::End(0)).unwrap() / N as u64
    }

    pub fn read(&self, start: u64, end: u64) -> Vec<[u8; N]> {
        let mut buf: [u8; N] = [0; N];
        let mut ret = Vec::new();
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(start)).unwrap();
        let times = (end - start) / N as u64;
        for _ in 0..times {
            let n = file.read(&mut buf).unwrap();
            if n != 8 {
                break;
            }
            ret.push(buf);
        }
        ret
    }

    pub fn read_to_end(&self, start: u64) -> Vec<[u8; N]> {
        let mut buf: [u8; N] = [0; N];
        let mut ret = Vec::new();
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(start)).unwrap();
        loop {
            let n = file.read(&mut buf).unwrap();
            if n != N {
                break;
            }
            ret.push(buf);
        }
        ret
    }
}
