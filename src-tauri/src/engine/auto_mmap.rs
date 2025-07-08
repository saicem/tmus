use std::fs::File;
use std::mem::size_of;

use memmap2::MmapMut;

pub struct AutoMmap {
    size: usize,
    len: usize,
    mmap: MmapMut,
    file: File,
}

static META_OFFSET: usize = size_of::<u64>();
static EXPAND_SIZE: usize = 8 * 1024 * 4;

impl AutoMmap {
    pub(crate) fn new(filepath: &str) -> Self {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(filepath)
            .expect(&format!("Failed to open file {}", filepath));
        let size = file.metadata().unwrap().len();
        if size < EXPAND_SIZE as u64 {
            file.set_len(EXPAND_SIZE as u64)
                .expect("Failed to resize file length");
        }
        let mmap =
            unsafe { MmapMut::map_mut(&file).expect(&format!("Failed to mmap file {}", filepath)) };
        let size = mmap.len().saturating_sub(META_OFFSET);
        let len = u64::from_le_bytes(mmap[0..META_OFFSET].try_into().unwrap()) as usize;

        AutoMmap {
            size,
            len,
            mmap,
            file,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn read(&self, start_inclusive: usize, end_exclusive: usize) -> &[u8] {
        debug_assert!(start_inclusive <= end_exclusive);
        debug_assert!(end_exclusive <= self.len);
        &self.mmap[start_inclusive + META_OFFSET..end_exclusive + META_OFFSET]
    }

    pub(crate) fn write(&mut self, start_inclusive: usize, end_exclusive: usize, data: &[u8]) {
        debug_assert!(start_inclusive <= end_exclusive);
        debug_assert!(end_exclusive - start_inclusive == data.len());
        if end_exclusive > self.size {
            self.expand_size(end_exclusive);
        }
        self.mmap[start_inclusive + META_OFFSET..end_exclusive + META_OFFSET].copy_from_slice(data);
        self.len = self.len.max(end_exclusive);
        self.mmap[0..META_OFFSET].copy_from_slice(&(self.len as u64).to_le_bytes());
    }

    pub(crate) fn append(&mut self, data: &[u8]) {
        self.write(self.len, self.len + data.len(), data);
    }

    fn expand_size(&mut self, required: usize) {
        let new_size = (required + META_OFFSET) / EXPAND_SIZE * (EXPAND_SIZE + 1);
        self.file
            .set_len(new_size as u64)
            .expect("Failed to resize file");
        self.size = new_size - META_OFFSET;
        self.mmap = unsafe { MmapMut::map_mut(&self.file).expect(&format!("Failed to mmap file")) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_auto_mmap_new() {
        let test_file = "test_auto_mmap_new.aof";
        let _ = AutoMmap::new(test_file);
        assert!(Path::new(test_file).exists());
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_auto_mmap_read_write() {
        let test_file = "test_auto_mmap_read_write.aof";
        let mut mmap = AutoMmap::new(test_file);
        let data = b"test models";
        mmap.append(data);
        let read_data = mmap.read(0, data.len());
        assert_eq!(read_data, data);
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_auto_mmap_expand_size() {
        let test_file = "test_auto_mmap_expand_size.aof";
        let mut mmap = AutoMmap::new(test_file);
        let original_size = mmap.size;
        let large_data = vec![0u8; EXPAND_SIZE * 2];
        mmap.append(&large_data);
        assert!(mmap.size > original_size);
        fs::remove_file(test_file).unwrap();
    }
}
