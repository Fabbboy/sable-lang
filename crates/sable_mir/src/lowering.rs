use std::{cell::RefCell, collections::HashMap, rc::Rc};

use sable_parser::ast::{ast::AST, function::Function};

use crate::{error::LoweringError, mir::module::MirModule};

pub struct Lowerer<'ctx, 'l> {
  mir_mod: &'l mut MirModule<'ctx>,
  ast: Rc<RefCell<AST<'ctx>>>,
  errors: Vec<LoweringError>,
}

impl<'ctx, 'l> Lowerer<'ctx, 'l> {
  pub fn new(mir_mod: &'l mut MirModule<'ctx>, ast: Rc<RefCell<AST<'ctx>>>) -> Self {
    Self {
      mir_mod,
      ast,
      errors: Vec::new(),
    }
  }

  pub fn lower_func(&mut self, func: Rc<Function>) -> Result<(), Vec<LoweringError>> {
    Ok(())
  }

  pub fn lower(&mut self) -> Result<(), &[LoweringError]> {
    let ast = self.ast.borrow();
    let funcs = ast.get_funcs();

    for func in funcs {}

    if self.errors.is_empty() {
      Ok(())
    } else {
      Err(&self.errors)
    }
  }
}
