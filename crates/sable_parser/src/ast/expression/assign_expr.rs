use crate::position::Position;

use super::Expression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssignExpression<'s> {
  asignee: Option<&'s str>,
  value: Box<Expression<'s>>,
  pos: Position,
}

impl<'s> AssignExpression<'s> {
  pub fn new(asignee: Option<&'s str>, value: Expression<'s>, pos: Position) -> Self {
    Self {
      asignee,
      value: Box::new(value),
      pos,
    }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_asignee(&self) -> Option<&'s str> {
    self.asignee
  }

  pub fn get_value(&self) -> &Expression<'s> {
    &self.value
  }
}
