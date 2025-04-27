use crate::position::Position;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct NullExpression {
  pos: Position,
}

impl NullExpression {
  pub fn new(pos: Position) -> Self {
    Self { pos }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }
}
