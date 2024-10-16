use crate::engine::data::AppId;
use super::millisecond::Millisecond;

pub struct RecordsQuery {
    id: AppId,
    start: Millisecond,
    end: Millisecond,
}