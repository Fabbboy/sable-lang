use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
  checks::stmt_check::check_stmt,
  error::{func_already_defined::FunctionAlreadyDefined, AnalyzerError},
  resolver::Resolver, scope::NamendValue,
};
use sable_parser::ast::{ast::AST, expression::BlockExpression, function::Function};

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

  fn check_block<'f>(
    &mut self,
    block: &BlockExpression<'s>,
    f: Rc<Function<'s>>,
  ) -> Result<(), Vec<AnalyzerError<'s>>> {
    let mut errors = Vec::new();
    for (_, stmt) in block.get_stmts().iter().enumerate() {
      match check_stmt(self, stmt, f.clone()) {
        Ok(_) => {}
        Err(err) => errors.push(err),
      }
    }

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

    self.resolver.enter_scope();
    for arg in f.get_params() {
      let namend = NamendValue::new(
        arg.get_val_type().clone(),
        arg.get_pos().clone(),
      );
      self.resolver.define_var(arg.get_name(),  namend);
    }

    let block = f.get_body();
    let res = self.check_block(block, f.clone());

    self.resolver.exit_scope();

    return match res {
      Ok(_) => {
        self.funcs.insert(f.get_name(), i);
        Ok(())
      }
      Err(errs) => Err(errs),
    };
  }

  pub fn analyze(&mut self) -> Result<(), &Vec<AnalyzerError<'s>>> {
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
      Err(&self.errors)
    }
  }
}
