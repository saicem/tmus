use super::millisecond::Millisecond;

/// Options for optimize storage
///
/// # Fields
///
/// * `keep_apps`: The ids of apps which records should be kept.
/// * `remove_apps`: The ids of apps which records should be removed.
/// * `start_time`: The start time of the time period which records should be kept.
/// * `end_time`: The end time of the time period which records should be kept.
pub struct OptimizeStorageOptions {
    keep_apps: Vec<usize>,
    remove_apps: Vec<usize>,
    start_time: Millisecond,
    end_time: Millisecond,
}