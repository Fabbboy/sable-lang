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

  pub fn merge(&self, other: Position) -> Self {
    let start = self.range.start.min(other.range.start);
    let end = self.range.end.max(other.range.end);
    Self {
      line: self.line,
      column: self.column,
      range: start..end,
    }
  }
}
