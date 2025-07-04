#[derive(Debug)]
#[derive(PartialEq)]
pub enum CursorPosition {
    Start,
    End,
    Middle(usize),
}
