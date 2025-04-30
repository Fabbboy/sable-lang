use crate::{info::OperatorType, position::Position};

use super::Expression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BinaryExpression<'s> {
  left: Box<Expression<'s>>,
  operator: OperatorType,
  right: Box<Expression<'s>>,
  pos: Position,
}

impl<'s> BinaryExpression<'s> {
  pub fn new(
    left: Expression<'s>,
    operator: OperatorType,
    right: Expression<'s>,
    pos: Position,
  ) -> Self {
    Self {
      left: Box::new(left),
      operator,
      right: Box::new(right),
      pos,
    }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_left_mut(&mut self) -> &mut Expression<'s> {
    &mut self.left
  }

  pub fn get_operator(&self) -> &OperatorType {
    &self.operator
  }

  pub fn get_right_mut(&mut self) -> &mut Expression<'s> {
    &mut self.right
  }

  pub fn get_left(&self) -> &Expression<'s> {
    &self.left
  }

  pub fn get_right(&self) -> &Expression<'s> {
    &self.right
  }
}
