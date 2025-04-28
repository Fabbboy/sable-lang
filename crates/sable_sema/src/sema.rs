use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
  error::{func_already_defined::FunctionAlreadyDefined, var_redeclared::VariableRedeclared, AnalyzerError},
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
  resolver: Resolver<'s>,
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

  fn check_let(
    &mut self,
    let_statement: &LetStatement<'s>,
    i: usize,
    f: Rc<Function<'s>>,
  ) -> Result<(), AnalyzerError<'s>> {
    let name = let_statement.get_name();
    if self.resolver.is_declared(name) {
      let earlier = match self.resolver.resolve_var(name) {
        Some(v) => v,
        None => {
          unreachable!()
        }
      };

      return Err(AnalyzerError::VariableRedeclared(
        VariableRedeclared::new(
          name,
          let_statement.get_pos().clone(),
          earlier.get_pos(f).clone(),
        ),
      ));
    }

    let namend = NamendValue::LetStmt(i);
    self.resolver.define_var(name, namend);
    Ok(())
  }
 
  fn check_statement(
    &mut self,
    stmt: &Statement<'s>,
    i: usize,
    f: Rc<Function<'s>>,
  ) -> Result<(), AnalyzerError<'s>> {
    match stmt {
      Statement::Expression(expression) => Ok(()),
      Statement::ReturnStatement(return_statement) => Ok(()),
      Statement::LetStatement(let_statement) => self.check_let(let_statement, i, f),
    }
  }

  fn check_block(
    &mut self,
    block: &BlockExpression<'s>,
    f: Rc<Function<'s>>,
  ) -> Result<(), Vec<AnalyzerError<'s>>> {
    let mut errors = Vec::new();
    self.resolver.enter_scope();
    for (i, stmt) in block.get_stmts().iter().enumerate() {
      match self.check_statement(stmt, i, f.clone()) {
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
    self.funcs.insert(f.get_name(), i);

    let block = f.get_body();
    let res = self.check_block(block, f.clone());
    return match res {
      Ok(_) => Ok(()),
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
