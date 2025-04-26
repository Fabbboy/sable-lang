use crate::{info::ValType, position::Position};

#[derive(Debug)]
pub struct LiteralExpression<'s> {
  value: &'s str,
  type_: ValType,
  pos: Position,
}

impl<'s> LiteralExpression<'s> {
  pub fn new(value: &'s str, type_: ValType, pos: Position) -> Self {
    Self { value, type_, pos }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_value(&self) -> &'s str {
    self.value
  }
  pub fn get_type(&self) -> ValType {
    self.type_.clone()
  }
}
