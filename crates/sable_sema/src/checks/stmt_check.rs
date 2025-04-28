use sable_parser::ast::statement::{LetStatement, Statement};

use crate::{
  error::{AnalyzerError, var_redeclared::VariableRedeclared},
  scope::NamendValue,
  sema::Sema,
};

use super::expr_check::check_expr;

pub fn check_stmt<'s>(
  analyzer: &mut Sema<'s>,
  stmt: &Statement<'s>,
) -> Result<(), AnalyzerError<'s>> {
  match stmt {
    Statement::Expression(expression) => check_expr(analyzer, expression),
    Statement::ReturnStatement(_) => Ok(()),
    Statement::LetStatement(let_statement) => check_let_stmt(analyzer, let_statement),
  }
}

pub fn check_let_stmt<'s>(
  analyzer: &mut Sema<'s>,
  let_statement: &LetStatement<'s>,
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
    check_expr(analyzer, assignee.get_value())?;
  }

  let namend = NamendValue::new(
    let_statement.get_type().clone(),
    let_statement.get_pos().clone(),
  );

  analyzer.resolver.define_var(name, namend);
  Ok(())
}
