use sable_parser::ast::expression::Expression;

use crate::{error::AnalyzerError, sema::Sema};

pub fn check_expr<'s>(
  analyzer: &mut Sema<'s>,
  expr: &Expression<'s>,
) -> Result<(), AnalyzerError<'s>> {
  match expr {
    Expression::LiteralExpression(literal_expression) => todo!(),
    Expression::BlockExpression(block_expression) => todo!(),
    Expression::AssignExpression(assign_expression) => todo!(),
    Expression::VariableExpression(variable_expression) => todo!(),
    Expression::BinaryExpression(binary_expression) => todo!(),
    Expression::NullExpression(null_expression) => todo!(),
  }
}
