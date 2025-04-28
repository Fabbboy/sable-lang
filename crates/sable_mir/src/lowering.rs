use std::{cell::RefCell, rc::Rc};

use sable_parser::{
  ast::{ast::AST, function::Function},
  info::ValType,
};

use crate::{
  error::{LoweringError, VoidIllegal},
  mir::{
    function::{MirFunction, MirParam},
    module::MirModule,
  },
};

pub struct Lowerer<'l, 's> {
  module: &'l mut MirModule<'s>,
  ast: Rc<RefCell<AST<'s>>>,
  errors: Vec<LoweringError>,
}

impl<'l, 's> Lowerer<'l, 's> {
  pub fn new(module: &'l mut MirModule<'s>, ast: Rc<RefCell<AST<'s>>>) -> Self {
    Self {
      module,
      ast,
      errors: Vec::new(),
    }
  }

  fn type_sanity_check(type_: ValType) -> bool {
    match type_ {
      ValType::Void => false,
      _ => true,
    }
  }

  fn lower_function(
    &mut self,
    function: Rc<Function<'s>>,
  ) -> Result<MirFunction<'s>, Vec<LoweringError>> {
    let errs = Vec::new();

    let name = function.get_name();
    let mut mir_func = MirFunction::new(name);
    for arg in function.get_params() {
      let name = arg.get_name();
      let type_ = arg.get_val_type();
      if !Self::type_sanity_check(type_.clone()) {
        let err = LoweringError::VoidIllegal(VoidIllegal::new(arg.get_pos()));
        self.errors.push(err);
        continue;
      }

      let param = MirParam::new(name, type_);
      mir_func.add_argument(param);
    }

    if errs.is_empty() {
      Ok(mir_func)
    } else {
      Err(errs)
    }
  }

  pub fn lower(&mut self) -> Result<(), &Vec<LoweringError>> {
    let functions = {
      let ast_ref = self.ast.borrow();
      ast_ref.get_funcs().to_vec()
    };

    for function in functions {
      match self.lower_function(function) {
        Ok(mir_func) => {
          self.module.add_function(mir_func);
        }
        Err(errs) => {
          self.errors.extend(errs);
        }
      }
    }

    if self.errors.is_empty() {
      Ok(())
    } else {
      Err(&self.errors)
    }
  }
}
