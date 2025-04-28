use std::rc::Rc;

use sable_parser::{
  ast::{
    function::Function,
    statement::{LetStatement, Statement},
  },
  info::ValType,
};

use crate::{
  error::{
    AnalyzerError, SemaExprError, VariableRedeclared,
    expr_errs::{IllegalNullUntyped, TypeMismatch},
  },
  scope::NamendValue,
  sema::Sema,
};

use super::{expr_check::check_expr, inference::infer_expr};

pub fn check_stmt<'s>(
  analyzer: &mut Sema<'s>,
  stmt: &Statement<'s>,
  f: Rc<Function<'s>>,
) -> Result<(), AnalyzerError<'s>> {
  match stmt {
    Statement::Expression(expression) => check_expr(analyzer, expression, f),
    Statement::ReturnStatement(ret_statement) => check_ret_stmt(analyzer, ret_statement, f),
    Statement::LetStatement(let_statement) => check_let_stmt(analyzer, let_statement, f),
  }
}

pub fn check_ret_stmt<'s>(
  analyzer: &mut Sema<'s>,
  ret_statement: &sable_parser::ast::statement::ReturnStatement<'s>,
  f: Rc<Function<'s>>,
) -> Result<(), AnalyzerError<'s>> {
  let value = ret_statement.get_value();
  check_expr(analyzer, &value, f.clone())?;
  let val_type = infer_expr(analyzer, value);
  if val_type == ValType::Void || val_type == ValType::Untyped {
    return Err(AnalyzerError::ExprError(SemaExprError::IllegalNullVoid(
      IllegalNullUntyped::new(value.get_pos().clone()),
    )));
  }
  if val_type != f.get_ret_type() {
    return Err(AnalyzerError::ExprError(SemaExprError::TypeMismatch(
      TypeMismatch::new(f.get_ret_type().clone(), val_type, value.get_pos().clone()),
    )));
  }

  Ok(())
}

pub fn check_let_stmt<'s>(
  analyzer: &mut Sema<'s>,
  let_statement: &LetStatement<'s>,
  f: Rc<Function<'s>>,
) -> Result<(), AnalyzerError<'s>> {
  let name = let_statement.get_name();
  if analyzer.resolver.is_declared(name) {
    let earlier = match analyzer.resolver.resolve_var(name) {
      Some(v) => v,
      None => {
        unreachable!()
      }
    };

    return Err(AnalyzerError::VariableRedeclared(VariableRedeclared::new(
      name,
      let_statement.get_pos().clone(),
      earlier.get_pos().clone(),
    )));
  }

  if let Some(assignee) = let_statement.get_assignee() {
    check_expr(analyzer, assignee.get_value(), f)?;
    let val_type = infer_expr(analyzer, assignee.get_value());
    if val_type == ValType::Void || val_type == ValType::Untyped {
      return Err(AnalyzerError::ExprError(SemaExprError::IllegalNullVoid(
        IllegalNullUntyped::new(assignee.get_pos().clone()),
      )));
    }

    if val_type != let_statement.get_type().clone() {
      return Err(AnalyzerError::ExprError(SemaExprError::TypeMismatch(
        TypeMismatch::new(
          let_statement.get_type().clone(),
          val_type,
          assignee.get_pos().clone(),
        ),
      )));
    }
  }

  let namend = NamendValue::new(
    let_statement.get_type().clone(),
    let_statement.get_pos().clone(),
  );

  analyzer.resolver.define_var(name, namend);
  Ok(())
}
