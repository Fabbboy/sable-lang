use std::{cell::RefCell, rc::Rc};

use sable_parser::ast::ast::AST;

use crate::{error::LoweringError, mir::module::MirModule};

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

  pub fn lower(&mut self) -> Result<(), &Vec<LoweringError>> {
    if self.errors.is_empty() {
      Ok(())
    } else {
      Err(&self.errors)
    }
  }
}
