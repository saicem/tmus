use crate::models::RecordByte;
use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use tracing::debug;

const RECORD_SIZE: usize = size_of::<RecordByte>();
/// The size use for mmap expand every time.
const EXPAND_SIZE: usize = 4 * 1024;

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

#[derive(Debug)]
pub struct State {
    file_path: PathBuf,
    mmap: MmapMut,
    len: usize,
    size: usize,
}

fn get_state<'a>() -> MutexGuard<'a, State> {
    STATE.get().unwrap().lock().unwrap()
}

pub fn init(data_dir: &PathBuf) {
    let file_path = data_dir.join("record.bin");
    let mmap = map_file(&file_path, None);
    let size = mmap.len() / 8 * 8;
    let search_start = size.saturating_sub(4 * 1024);
    let len = search_start + find_really_len(&mmap[search_start..size]);
    STATE
        .set(Mutex::new(State {
            file_path,
            mmap,
            len,
            size,
        }))
        .unwrap();
}

pub fn write(record: RecordByte) -> u64 {
    let mut state = get_state();
    if state.size <= state.len {
        expand_size(&mut state);
    }
    let range = state.len..state.len + RECORD_SIZE;
    state.mmap[range].copy_from_slice(&record);
    state.len += 8;
    debug!(
        "write record:{}",
        record
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    );
    (state.len / RECORD_SIZE) as u64
}

pub fn read(start: Option<usize>, end: Option<usize>) -> Vec<RecordByte> {
    let state = get_state();
    let start = start.map_or(0, |start| start * RECORD_SIZE);
    let end = end.map_or(state.len, |end| {
        debug_assert!(end * RECORD_SIZE <= state.len);
        end * RECORD_SIZE
    });
    debug_assert!(start < end, "start must less than end.");
    state.mmap[start..end]
        .chunks(RECORD_SIZE)
        .map(|chunk| chunk.try_into().unwrap())
        .collect()
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

fn expand_size(state: &mut State) {
    let new_size = state.size + EXPAND_SIZE;
    state.mmap = map_file(&state.file_path, Some(new_size));
    state.size = new_size;
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
