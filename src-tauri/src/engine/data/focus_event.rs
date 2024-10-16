use super::millisecond::Millisecond;

#[derive(Debug)]
pub struct FocusEvent {
    pub(crate) app_path: String,
    pub(crate) focus_at: Millisecond,
}
