use super::millisecond::Millisecond;
use crate::engine::data::AppId;

pub struct RecordsQuery {
    id: AppId,
    start: Millisecond,
    end: Millisecond,
}
