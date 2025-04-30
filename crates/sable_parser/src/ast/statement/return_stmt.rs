use crate::{ast::expression::Expression, info::ValType, position::Position};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReturnStatement<'s> {
  value: Expression<'s>,
  type_: ValType,
  pos: Position,
}

impl<'s> ReturnStatement<'s> {
  pub fn new(value: Expression<'s>, type_: ValType, pos: Position) -> Self {
    Self { value, type_, pos }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_value(&self) -> &Expression<'s> {
    &self.value
  }

  pub fn get_value_mut(&mut self) -> &mut Expression<'s> {
    &mut self.value
  }

  pub fn get_type(&self) -> ValType {
    self.type_.clone()
  }
}
