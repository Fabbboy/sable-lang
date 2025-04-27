use crate::position::Position;

use super::expression;

pub mod return_stmt;
pub mod let_stmt;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Statement<'s> {
  Expression(expression::Expression<'s>),
  ReturnStatement(return_stmt::ReturnStatement<'s>),
  LetStatement(let_stmt::LetStatement<'s>),
}

impl<'s> Statement<'s> {
  pub fn get_pos(&self) -> Position {
    match self {
      Statement::Expression(expr) => expr.get_pos(),
      Statement::ReturnStatement(stmt) => stmt.get_pos(),
      Statement::LetStatement(stmt) => stmt.get_pos(),
    }
  }
}
