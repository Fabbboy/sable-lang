use std::{
  cell::RefCell,
  rc::Rc,
  vec,
};

use super::function::MirFunction;

#[derive(Debug)]
pub struct MirModule<'s> {
  name: &'s str,
  funcs: Vec<Rc<RefCell<MirFunction<'s>>>>,
}

impl<'s> MirModule<'s> {
  pub fn new(name: &'s str) -> Self {
    Self {
      name,
      funcs: vec![],
    }
  }

  pub fn add_func(&mut self, func: MirFunction<'s>) {
    let func = Rc::new(RefCell::new(func));
    self.funcs.push(func);
  }

  pub fn name(&self) -> &str {
    self.name
  }
}
