use std::{cell::RefCell, rc::Rc};

use sable_parser::ast::{
  expression::{
    AssignExpression, BinaryExpression, BlockExpression, CallExpression, Expression,
    VariableExpression,
  },
  function::Function,
};

use crate::{
  checks::inference::infer_expr,
  error::{
    AnalyzerError,
    expr_errs::{ExprCheckError, TypeMismatch, VariableNotFound},
    func_checks::{FunctionArgumentMismatch, FunctionCheckError, FunctionNotFound},
  },
  sema::Sema,
};

use super::stmt_check::check_stmt;

pub fn check_expr<'s>(
  analyzer: &mut Sema<'s>,
  expr: &mut Expression<'s>,
  f: Rc<RefCell<Function<'s>>>,
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
    Expression::CallExpression(call_expression) => {
      check_call_expression(analyzer, call_expression, f)
    }
  }
}

pub fn check_call_expression<'s>(
  analyzer: &mut Sema<'s>,
  call_expression: &mut CallExpression<'s>,
  f: Rc<RefCell<Function<'s>>>,
) -> Result<(), AnalyzerError<'s>> {
  let name = call_expression.get_callee();
  let func_idx = analyzer.funcs.get(name);
  if func_idx.is_none() {
    return Err(AnalyzerError::FuncError(
      FunctionCheckError::FunctionNotFound(FunctionNotFound::new(
        name,
        call_expression.get_pos().clone(),
      )),
    ));
  }

  let func = analyzer.get_func(*func_idx.unwrap());
  let args = call_expression.get_args_mut();
  let binding = func.borrow();
  let params = binding.get_params();

  if args.len() != params.len() {
    return Err(AnalyzerError::FuncError(
      FunctionCheckError::FunctionArgumentMismatch(FunctionArgumentMismatch::new(
        func.borrow().get_name(),
        params.len(),
        args.len(),
        call_expression.get_pos().clone(),
      )),
    ));
  }

  for (i, arg) in args.iter_mut().enumerate() {
    let res = check_expr(analyzer, arg, f.clone());
    if res.is_err() {
      return res;
    }
    let arg_type = infer_expr(analyzer, arg);
    let param_type = params[i].get_val_type();
    if arg_type != param_type {
      return Err(AnalyzerError::ExprError(ExprCheckError::TypeMismatch(
        TypeMismatch::new(param_type.clone(), arg_type, arg.get_pos().clone()),
      )));
    }
  }

  Ok(())
}

pub fn check_block_expression<'s>(
  analyzer: &mut Sema<'s>,
  block_expression: &mut BlockExpression<'s>,
  f: Rc<RefCell<Function<'s>>>,
) -> Result<(), AnalyzerError<'s>> {
  analyzer.resolver.enter_scope();
  for (_, stmt) in block_expression.get_stmts_mut().iter_mut().enumerate() {
    match check_stmt(analyzer, stmt, f.clone()) {
      Ok(_) => {}
      Err(err) => return Err(err),
    }
  }
  Ok(())
}

pub fn check_binary_expression<'s>(
  analyzer: &mut Sema<'s>,
  binary_expression: &mut BinaryExpression<'s>,
  f: Rc<RefCell<Function<'s>>>,
) -> Result<(), AnalyzerError<'s>> {
  let lhs_checked = check_expr(analyzer, binary_expression.get_left_mut(), f.clone());
  let rhs_checked = check_expr(analyzer, binary_expression.get_right_mut(), f);

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

  Err(AnalyzerError::ExprError(ExprCheckError::TypeMismatch(
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
    Err(AnalyzerError::ExprError(ExprCheckError::VariableNotFound(
      VariableNotFound::new(name, variable_expression.get_pos().clone()),
    )))
  }
}

pub fn check_assign_expression<'s>(
  analyzer: &mut Sema<'s>,
  assign_expression: &mut AssignExpression<'s>,
  f: Rc<RefCell<Function<'s>>>,
) -> Result<(), AnalyzerError<'s>> {
  return check_expr(analyzer, assign_expression.get_value_mut(), f);
}
