use crate::position::Position;

use super::expression;

pub mod return_stmt;

#[derive(Debug)]
pub enum Statement<'s> {
  Expression(expression::Expression<'s>),
  ReturnStatement(return_stmt::ReturnStatement<'s>),
}

impl<'s> Statement<'s> {
  pub fn get_pos(&self) -> Position {
    match self {
      Statement::Expression(expr) => expr.get_pos(),
      Statement::ReturnStatement(stmt) => stmt.get_pos(),
    }
  }
}
