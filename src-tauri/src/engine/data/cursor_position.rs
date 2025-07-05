#[derive(Debug, PartialEq)]
pub enum CursorPosition {
    Start,
    End,
    Middle(usize),
}
