use crate::{info::ValType, position::Position};

use super::expression::block_expr::BlockExpression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Function<'s> {
  name: &'s str,
  pos: Position,
  ret_type: ValType,
  body: BlockExpression<'s>,
}

impl<'s> Function<'s> {
  pub fn new(name: &'s str, pos: Position, ret_type: ValType, body: BlockExpression<'s>) -> Self {
    Self {
      name,
      pos,
      ret_type,
      body,
    }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }

  pub fn get_ret_type(&self) -> ValType {
    self.ret_type.clone()
  }

  pub fn get_body(&self) -> &BlockExpression<'s> {
    &self.body
  }
}
