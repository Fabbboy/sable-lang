use crate::position::Position;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VariableExpression<'s> {
  name: &'s str,
  pos: Position,
}

impl<'s> VariableExpression<'s> {
  pub fn new(name: &'s str, pos: Position) -> Self {
    Self { name, pos }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }
}
