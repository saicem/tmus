use crate::models::RecordByte;
use memmap2::{Mmap, MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use tracing::debug;

const RECORD_SIZE: usize = size_of::<RecordByte>();
/// The size use for mmap expand every time.
const EXPAND_SIZE: usize = 4 * 1024;

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

/// A record.bin mapping. The writable variant is owned by the GUI tracker,
/// the read-only variant is used by short-lived CLI processes so they never
/// conflict with the GUI's writable mapping.
#[derive(Debug)]
enum MmapHandle {
    Writable(MmapMut),
    ReadOnly(Mmap),
    Empty,
}

#[derive(Debug)]
pub struct State {
    file_path: PathBuf,
    mmap: MmapHandle,
    len: usize,
    size: usize,
    read_only: bool,
}

fn get_state<'a>() -> MutexGuard<'a, State> {
    STATE.get().unwrap().lock().unwrap()
}

pub fn init(data_dir: impl AsRef<Path>) {
    init_inner(data_dir, false);
}

/// Initialize record storage in read-only mode.
/// A missing or empty record.bin is treated as empty data.
pub fn init_readonly(data_dir: impl AsRef<Path>) {
    init_inner(data_dir, true);
}

fn init_inner(data_dir: impl AsRef<Path>, read_only: bool) {
    let file_path = data_dir.as_ref().join("record.bin");
    let mmap = map_file(&file_path, None, read_only);
    let size = mmap_len(&mmap) / 8 * 8;
    let search_start = size.saturating_sub(4 * 1024);
    let len = if size == 0 {
        0
    } else {
        search_start + find_really_len(&mmap_slice(&mmap)[search_start..size])
    };
    STATE
        .set(Mutex::new(State {
            file_path,
            mmap,
            len,
            size,
            read_only,
        }))
        .unwrap();
}

fn mmap_len(mmap: &MmapHandle) -> usize {
    match mmap {
        MmapHandle::Writable(m) => m.len(),
        MmapHandle::ReadOnly(m) => m.len(),
        MmapHandle::Empty => 0,
    }
}

fn mmap_slice(mmap: &MmapHandle) -> &[u8] {
    match mmap {
        MmapHandle::Writable(m) => m,
        MmapHandle::ReadOnly(m) => m,
        MmapHandle::Empty => &[],
    }
}

pub fn write(record: RecordByte) -> u64 {
    let mut state = get_state();
    assert!(!state.read_only, "Cannot write to read-only storage.");
    if state.size <= state.len {
        expand_size(&mut state);
    }
    let range = state.len..state.len + RECORD_SIZE;
    match &mut state.mmap {
        MmapHandle::Writable(mmap) => mmap[range].copy_from_slice(&record),
        _ => unreachable!("writable storage must have a writable mapping"),
    }
    state.len += 8;
    debug!(
        "Write record:{}",
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
    debug_assert!(start <= end, "Start must less than or equal to end.");
    mmap_slice(&state.mmap)[start..end]
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
    assert!(!state.read_only, "Cannot expand read-only storage.");
    let new_size = state.size + EXPAND_SIZE;
    state.mmap = map_file(&state.file_path, Some(new_size), false);
    state.size = new_size;
}

fn map_file<T: AsRef<Path>>(file_path: T, size: Option<usize>, read_only: bool) -> MmapHandle {
    if read_only {
        let Ok(file) = OpenOptions::new().read(true).open(file_path) else {
            return MmapHandle::Empty;
        };
        return match unsafe { MmapOptions::new().map(&file) } {
            Ok(mmap) => MmapHandle::ReadOnly(mmap),
            Err(_) => MmapHandle::Empty,
        };
    }

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(file_path)
        .expect("open record.bin failed.");
    if let Some(size) = size {
        file.set_len(size as u64).expect("Resize file failed.");
    }
    unsafe { MmapHandle::Writable(MmapMut::map_mut(&file).expect("Error mapping record.bin file")) }
}
