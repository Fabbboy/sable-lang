use crate::position::Position;

pub mod literal_expr;
pub mod block_expr;

#[derive(Debug)]
pub enum Expression<'s> {
  LiteralExpression(literal_expr::LiteralExpression<'s>),
  BlockExpression(block_expr::BlockExpression<'s>),
}

impl<'s> Expression<'s> {
  pub fn get_pos(&self) -> Position {
    match self {
      Expression::LiteralExpression(expr) => expr.get_pos(),
      Expression::BlockExpression(expr) => expr.get_pos(),
    }
  }
}
