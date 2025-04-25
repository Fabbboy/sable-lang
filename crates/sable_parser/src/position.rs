use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position {
  pub line: usize,
  pub column: usize,
  pub range: Range<usize>,
}

impl Position {
  pub fn new(line: usize, column: usize, range: Range<usize>) -> Self {
    Self {
      line,
      column,
      range,
    }
  }
}
