use std::{cell::RefCell, rc::Rc};

use super::function::MirFunction;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirModule<'s> {
  name: &'s str,
  functions: Vec<Rc<RefCell<MirFunction<'s>>>>,
}

impl<'s> MirModule<'s> {
  pub fn new(name: &'s str) -> Self {
    Self {
      name,
      functions: Vec::new(),
    }
  }

  pub fn add_function(&mut self, function: MirFunction<'s>) -> Rc<RefCell<MirFunction<'s>>> {
    let function = Rc::new(RefCell::new(function));
    self.functions.push(function.clone());
    function
  }

  pub fn get_functions(&self) -> &[Rc<RefCell<MirFunction<'s>>>] {
    &self.functions
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }
}
