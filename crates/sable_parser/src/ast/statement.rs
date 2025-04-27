use crate::position::Position;

use super::expression;

pub mod return_stmt;
pub mod var_decl_stmt;

#[derive(Debug)]
pub enum Statement<'s> {
  Expression(expression::Expression<'s>),
  ReturnStatement(return_stmt::ReturnStatement<'s>),
  VariableDeclStatement(var_decl_stmt::VariableDeclStatement<'s>),
}

impl<'s> Statement<'s> {
  pub fn get_pos(&self) -> Position {
    match self {
      Statement::Expression(expr) => expr.get_pos(),
      Statement::ReturnStatement(stmt) => stmt.get_pos(),
      Statement::VariableDeclStatement(stmt) => stmt.get_pos(),
    }
  }
}
