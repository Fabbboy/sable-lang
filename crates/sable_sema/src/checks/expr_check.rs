use std::rc::Rc;

use sable_parser::ast::{
  expression::{
    AssignExpression, BinaryExpression, BlockExpression, Expression, VariableExpression,
  },
  function::Function,
};

use crate::{
  checks::inference::infer_expr,
  error::{
    AnalyzerError,
    expr_errs::{SemaExprError, TypeMismatch, VariableNotFound},
  },
  sema::Sema,
};

use super::stmt_check::check_stmt;

pub fn check_expr<'s>(
  analyzer: &mut Sema<'s>,
  expr: &Expression<'s>,
  f: Rc<Function<'s>>,
) -> Result<(), AnalyzerError<'s>> {
  match expr {
    Expression::LiteralExpression(_) => Ok(()),
    Expression::BlockExpression(block_expression) => {
      check_block_expression(analyzer, block_expression, f.clone())
    }
    Expression::AssignExpression(assign_expression) => {
      check_assign_expression(analyzer, assign_expression, f.clone())
    }
    Expression::VariableExpression(variable_expression) => {
      check_variable_expression(analyzer, variable_expression)
    }
    Expression::BinaryExpression(binary_expression) => {
      check_binary_expression(analyzer, binary_expression, f.clone())
    }
    Expression::NullExpression(_) => Ok(()),
  }
}

pub fn check_block_expression<'s>(
  analyzer: &mut Sema<'s>,
  block_expression: &BlockExpression<'s>,
  f: Rc<Function<'s>>,
) -> Result<(), AnalyzerError<'s>> {
  analyzer.resolver.enter_scope();
  for (_, stmt) in block_expression.get_stmts().iter().enumerate() {
    match check_stmt(analyzer, stmt, f.clone()) {
      Ok(_) => {}
      Err(err) => return Err(err),
    }
  }
  Ok(())
}

pub fn check_binary_expression<'s>(
  analyzer: &mut Sema<'s>,
  binary_expression: &BinaryExpression<'s>,
  f: Rc<Function<'s>>,
) -> Result<(), AnalyzerError<'s>> {
  let lhs_checked = check_expr(analyzer, binary_expression.get_left(), f.clone());
  let rhs_checked = check_expr(analyzer, binary_expression.get_right(), f);

  if lhs_checked.is_err() {
    return lhs_checked;
  }

  if rhs_checked.is_err() {
    return rhs_checked;
  }

  let lhs = binary_expression.get_left();
  let rhs = binary_expression.get_right();
  let lhs_type = infer_expr(analyzer, lhs);
  let rhs_type = infer_expr(analyzer, rhs);

  if lhs_type == rhs_type {
    return Ok(());
  }

  Err(AnalyzerError::ExprError(SemaExprError::TypeMismatch(
    TypeMismatch::new(lhs_type, rhs_type, binary_expression.get_pos().clone()),
  )))
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
  f: Rc<Function<'s>>,
) -> Result<(), AnalyzerError<'s>> {
  return check_expr(analyzer, assign_expression.get_value(), f);
}
