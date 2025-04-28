use sable_parser::ast::expression::{
  AssignExpression, Expression, LiteralExpression, VariableExpression,
};

use crate::{
  error::{
    AnalyzerError,
    expr_errs::{SemaExprError, VariableNotFound},
  },
  sema::Sema,
};

pub fn check_expr<'s>(
  analyzer: &mut Sema<'s>,
  expr: &Expression<'s>,
) -> Result<(), AnalyzerError<'s>> {
  match expr {
    Expression::LiteralExpression(literal_expression) => Ok(()),
    Expression::BlockExpression(block_expression) => todo!(),
    Expression::AssignExpression(assign_expression) => {
      check_assign_expression(analyzer, assign_expression)
    }
    Expression::VariableExpression(variable_expression) => {
      check_variable_expression(analyzer, variable_expression)
    }
    Expression::BinaryExpression(binary_expression) => todo!(),
    Expression::NullExpression(null_expression) => Ok(()),
  }
}

pub fn check_variable_expression<'s>(
  analyzer: &mut Sema<'s>,
  variable_expression: &VariableExpression<'s>,
) -> Result<(), AnalyzerError<'s>> {
  let name = variable_expression.get_name();
  if analyzer.resolver.is_declared(name) {
    Ok(())
  } else {
    Err(AnalyzerError::ExprError(SemaExprError::VariableNotFound(
      VariableNotFound::new(name, variable_expression.get_pos().clone()),
    )))
  }
}

pub fn check_assign_expression<'s>(
  analyzer: &mut Sema<'s>,
  assign_expression: &AssignExpression<'s>,
) -> Result<(), AnalyzerError<'s>> {
  return check_expr(analyzer, assign_expression.get_value());
}
