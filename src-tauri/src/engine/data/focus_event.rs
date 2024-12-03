use super::millisecond::Millisecond;

/// An event that represents a window gaining focus.
///
/// It is used to pass information between the thread that listens for focus events and the thread that records the focus
/// events.
#[derive(Debug)]
pub struct FocusEvent {
    /// The path of the executable of the focused window.
    pub(crate) app_path: String,
    /// The time when the window gained focus.
    pub(crate) focus_at: Millisecond,
}
