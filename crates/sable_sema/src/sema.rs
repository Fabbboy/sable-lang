use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
  checks::stmt_check::check_stmt,
  error::{AnalyzerError, func_already_defined::FunctionAlreadyDefined},
  resolver::Resolver,
  scope::NamendValue,
};
use sable_parser::ast::{ast::AST, expression::BlockExpression, function::Function};

pub struct Sema<'s> {
  errors: Vec<AnalyzerError<'s>>,
  pub resolver: Resolver<'s>,
  pub funcs: HashMap<&'s str, usize>,
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

  pub fn get_func(&self, idx: usize) -> Rc<RefCell<Function<'s>>> {
    let ast = self.ast.borrow();
    ast.get_funcs()[idx].clone()
  }

  pub fn check_function(
    &mut self,
    f: Rc<RefCell<Function<'s>>>,
    func_idx: usize,
  ) -> Result<(), Vec<AnalyzerError<'s>>> {
    if self.funcs.contains_key(f.borrow().get_name()) {
      let earlier = self.funcs[f.borrow().get_name()];
      let earlier_func = self.get_func(earlier);
      return Err(vec![AnalyzerError::FunctionAlreadyDefined(
        FunctionAlreadyDefined::new(
          f.borrow().get_name(),
          earlier_func.borrow().get_pos().clone(),
          f.borrow().get_pos().clone(),
        ),
      )]);
    }
    self.funcs.insert(f.borrow().get_name(), func_idx);
    self.resolver.enter_scope();
    for param in f.borrow().get_params() {
      let nv = NamendValue::new(param.get_val_type().clone(), param.get_pos().clone());
      self.resolver.define_var(param.get_name(), nv);
    }

    let body_ptr: *mut BlockExpression<'s> = {
      let mut binding = f.borrow_mut();
      binding.get_body_mut() as *mut _
    };

    let result = unsafe {
      self.check_block(&mut *body_ptr, f.clone())
    };

    self.resolver.exit_scope();

    result
  }

  pub fn check_block(
    &mut self,
    block: &mut BlockExpression<'s>,
    f: Rc<RefCell<Function<'s>>>,
  ) -> Result<(), Vec<AnalyzerError<'s>>> {
    let mut errors = Vec::new();
    for stmt in block.get_stmts_mut().iter_mut() {
      if let Err(e) = check_stmt(self, stmt, f.clone()) {
        errors.push(e);
      }
    }
    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  pub fn analyze(&mut self) -> Result<(), &Vec<AnalyzerError<'s>>> {
    let func_entries: Vec<(usize, Rc<RefCell<Function<'s>>>)> = {
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
