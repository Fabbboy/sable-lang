use crate::{ast::statement::Statement, position::Position};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BlockExpression<'s> {
  pub stmts: Vec<Statement<'s>>,
  pub pos: Position,
}

impl<'s> BlockExpression<'s> {
  pub fn new(stmts: Vec<Statement<'s>>, pos: Position) -> Self {
    Self { stmts, pos }
  }

  pub fn get_pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn get_stmts(&self) -> &Vec<Statement<'s>> {
    &self.stmts
  }
}
