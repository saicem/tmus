use std::collections::HashMap;
use crate::app::file::FocusRecord;

pub fn collect_record(records: Vec<FocusRecord>) -> HashMap<u64, u64> {
    records
        .iter()
        .map(|x| (x.app_id(), x.duration()))
        .collect::<HashMap<_, _>>()
}
