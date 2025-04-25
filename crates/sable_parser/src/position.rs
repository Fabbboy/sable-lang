#[derive(Debug, Clone, PartialEq, Default)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub start: Range<usize>,
}

impl Position {
    pub fn new(line: usize, column: usize, start: Range<usize>) -> Self {
        Self {
            line,
            column,
            start,
        }
    }
}
