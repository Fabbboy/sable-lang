use crate::position::Position;

use super::Expression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CallExpression<'s> {
  pos: Position,
  callee: &'s str,
  args: Vec<Expression<'s>>,
}

impl<'s> CallExpression<'s> {
  pub fn new(pos: Position, callee: &'s str, args: Vec<Expression<'s>>) -> Self {
    Self { pos, callee, args }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_callee(&self) -> &'s str {
    self.callee
  }

  pub fn get_args(&self) -> &Vec<Expression<'s>> {
    &self.args
  }

  pub fn get_args_mut(&mut self) -> &mut Vec<Expression<'s>> {
    &mut self.args
  }
}
