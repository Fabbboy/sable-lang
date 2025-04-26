use crate::{ast::expression::Expression, position::Position};

#[derive(Debug)]
pub struct ReturnStatement<'s> {
  value: Expression<'s>,
  pos: Position,
}

impl<'s> ReturnStatement<'s> {
  pub fn new(value: Expression<'s>, pos: Position) -> Self {
    Self { value, pos }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_value(&self) -> &Expression<'s> {
    &self.value
  }
}
