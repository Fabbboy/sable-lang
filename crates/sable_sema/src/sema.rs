use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
  checks::expr_check::{check_assign_expression, check_expr},
  error::{
    AnalyzerError, func_already_defined::FunctionAlreadyDefined, var_redeclared::VariableRedeclared,
  },
  resolver::Resolver,
  scope::NamendValue,
};
use sable_parser::ast::{
  ast::AST,
  expression::BlockExpression,
  function::Function,
  statement::{LetStatement, Statement},
};

pub struct Sema<'s> {
  errors: Vec<AnalyzerError<'s>>,
  pub resolver: Resolver<'s>,
  funcs: HashMap<&'s str, usize>,
  ast: Rc<RefCell<AST<'s>>>,
}

impl<'s> Sema<'s> {
  pub fn new(ast: Rc<RefCell<AST<'s>>>) -> Self {
    Sema {
      errors: Vec::new(),
      resolver: Resolver::new(),
      funcs: HashMap::new(),
      ast,
    }
  }

  fn get_func(&self, idx: usize) -> Rc<Function<'s>> {
    let ast = self.ast.borrow();
    ast.get_funcs()[idx].clone()
  }

  fn check_let(&mut self, let_statement: &LetStatement<'s>) -> Result<(), AnalyzerError<'s>> {
    let name = let_statement.get_name();
    if self.resolver.is_declared(name) {
      let earlier = match self.resolver.resolve_var(name) {
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

    let namend = NamendValue::new(
      let_statement.get_type().clone(),
      let_statement.get_pos().clone(),
    );

    if let_statement.get_assignee().is_some() {
      let assignee = let_statement.get_assignee().as_ref().unwrap();
      let checked = check_assign_expression(self, assignee);
      match checked {
        Ok(_) => {}
        Err(err) => {
          return Err(err);
        }
      }
    }

    self.resolver.define_var(name, namend);
    Ok(())
  }

  fn check_statement(&mut self, stmt: &Statement<'s>) -> Result<(), AnalyzerError<'s>> {
    match stmt {
      Statement::Expression(expression) => {
        let checked = check_expr(self, expression);
        match checked {
          Ok(_) => Ok(()),
          Err(err) => Err(err),
        }
      }
      Statement::ReturnStatement(_) => Ok(()),
      Statement::LetStatement(let_statement) => self.check_let(let_statement),
    }
  }

  fn check_block<'f>(&mut self, block: &BlockExpression<'s>) -> Result<(), Vec<AnalyzerError<'s>>> {
    let mut errors = Vec::new();
    self.resolver.enter_scope();
    for (_, stmt) in block.get_stmts().iter().enumerate() {
      match self.check_statement(stmt) {
        Ok(_) => {}
        Err(err) => errors.push(err),
      }
    }

    self.resolver.exit_scope();
    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  fn check_function(
    &mut self,
    f: Rc<Function<'s>>,
    i: usize,
  ) -> Result<(), Vec<AnalyzerError<'s>>> {
    if self.funcs.contains_key(f.get_name()) {
      let earlier = self.funcs[f.get_name()];
      let earlier_func = self.get_func(earlier);
      return Err(vec![AnalyzerError::FunctionAlreadyDefined(
        FunctionAlreadyDefined::new(
          f.get_name(),
          f.get_pos().clone(),
          earlier_func.get_pos().clone(),
        ),
      )]);
    }

    let block = f.get_body();
    let res = self.check_block(block);

    return match res {
      Ok(_) => {
        self.funcs.insert(f.get_name(), i);
        Ok(())
      }
      Err(errs) => Err(errs),
    };
  }

  pub fn analyze(&mut self) -> Result<(), Vec<AnalyzerError<'s>>> {
    let func_entries: Vec<(usize, Rc<Function<'s>>)> = {
      let ast_ref = self.ast.borrow();
      ast_ref
        .get_funcs()
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, f_rc)| (i, f_rc))
        .collect()
    };

    for (i, f_rc) in func_entries {
      if let Err(errs) = self.check_function(f_rc, i) {
        self.errors.extend(errs);
      }
    }

    if self.errors.is_empty() {
      Ok(())
    } else {
      Err(std::mem::take(&mut self.errors))
    }
  }
}
