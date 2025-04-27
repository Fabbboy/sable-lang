use crate::{info::ValType, position::Position};

use super::expression::block_expr::BlockExpression;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FunctionParameter<'s> {
  name: &'s str,
  pos: Position,
  type_: ValType,
}

impl<'s> FunctionParameter<'s> {
  pub fn new(name: &'s str, pos: Position, val_type: ValType) -> Self {
    Self {
      name,
      pos,
      type_: val_type,
    }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }

  pub fn get_val_type(&self) -> ValType {
    self.type_.clone()
  }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Function<'s> {
  name: &'s str,
  pos: Position,
  ret_type: ValType,
  params: Vec<FunctionParameter<'s>>,
  body: BlockExpression<'s>,
}

impl<'s> Function<'s> {
  pub fn new(
    name: &'s str,
    params: Vec<FunctionParameter<'s>>,
    pos: Position,
    ret_type: ValType,
    body: BlockExpression<'s>,
  ) -> Self {
    Self {
      name,
      pos,
      ret_type,
      params,
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

  pub fn get_params(&self) -> &Vec<FunctionParameter<'s>> {
    &self.params
  }
}
