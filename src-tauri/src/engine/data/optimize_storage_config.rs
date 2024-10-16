use super::millisecond::Millisecond;

pub struct OptimizeStorageConfig {
    keep_apps: Vec<usize>,
    remove_apps: Vec<usize>,
    start_time: Millisecond,
    end_time: Millisecond,
}