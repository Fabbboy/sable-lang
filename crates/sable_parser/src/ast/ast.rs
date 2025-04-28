use std::rc::Rc;

use super::function::Function;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AST<'s> {
  functions: Vec<Rc<Function<'s>>>,
}

impl<'s> AST<'s> {
  pub fn new() -> Self {
    AST {
      functions: Vec::new(),
    }
  }

  pub fn add_func(&mut self, f: Function<'s>) {
    self.functions.push(Rc::new(f));
  }

  pub fn get_funcs(&self) -> &[Rc<Function<'s>>] {
    &self.functions
  }
}
