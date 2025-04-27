use crate::position::Position;

pub mod literal_expr;
pub mod block_expr;
pub mod assign_expr;
pub mod variable_expr;
pub mod binary_expr;
pub mod null_expr;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expression<'s> {
  LiteralExpression(literal_expr::LiteralExpression<'s>),
  BlockExpression(block_expr::BlockExpression<'s>),
  AssignExpression(assign_expr::AssignExpression<'s>),
  VariableExpression(variable_expr::VariableExpression<'s>),
  BinaryExpression(binary_expr::BinaryExpression<'s>),
  NullExpression(null_expr::NullExpression),
}

impl<'s> Expression<'s> {
  pub fn get_pos(&self) -> Position {
    match self {
      Expression::LiteralExpression(expr) => expr.get_pos(),
      Expression::BlockExpression(expr) => expr.get_pos(),
      Expression::AssignExpression(expr) => expr.get_pos(),
      Expression::VariableExpression(expr) => expr.get_pos(),
      Expression::BinaryExpression(expr) => expr.get_pos(),
      Expression::NullExpression(expr) => expr.get_pos(),
    }
  }
}
