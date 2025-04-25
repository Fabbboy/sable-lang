use crate::position::Position;

#[derive(Debug)]
pub struct Function<'s> {
  name: &'s str,
  pos: Position,
}

impl<'s> Function<'s> {
  pub fn new(name: &'s str, pos: Position) -> Self {
    Self {
      name,
      pos,
    }
  }
}
