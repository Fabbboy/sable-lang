use std::{cell::RefCell, rc::Rc};

use super::function::Function;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AST<'s> {
  functions: Vec<Rc<RefCell<Function<'s>>>>,
}

impl<'s> AST<'s> {
  pub fn new() -> Self {
    AST {
      functions: Vec::new(),
    }
  }

  pub fn add_func(&mut self, f: Function<'s>) {
    self.functions.push(Rc::new(RefCell::new(f)));
  }

  pub fn get_funcs(&self) -> &[Rc<RefCell<Function<'s>>>] {
    &self.functions
  }
}
