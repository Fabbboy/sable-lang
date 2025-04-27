use crate::position::Position;

use super::Expression;

#[derive(Debug)]
pub struct AssignExpression<'s> {
  asignee: &'s str,
  value: Box<Expression<'s>>,
  pos: Position,
}

impl<'s> AssignExpression<'s> {
  pub fn new(asignee: &'s str, value: Expression<'s>, pos: Position) -> Self {
    Self {
      asignee,
      value: Box::new(value),
      pos,
    }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_asignee(&self) -> &'s str {
    self.asignee
  }

  pub fn get_value(&self) -> &Expression<'s> {
    &self.value
  }
}
