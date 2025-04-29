use std::{
  cell::{Ref, RefCell, RefMut},
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

  pub fn add_func(&mut self, func: MirFunction<'s>) -> usize {
    let func = Rc::new(RefCell::new(func));
    self.funcs.push(func);
    self.funcs.len() - 1
  }

  pub fn get_func(&self, index: usize) -> Option<Ref<MirFunction<'s>>> {
    self.funcs.get(index).map(|cell| cell.borrow())
  }

  pub fn get_func_mut(&mut self, index: usize) -> Option<RefMut<MirFunction<'s>>> {
    self.funcs.get_mut(index).map(|cell| cell.borrow_mut())
  }

  pub fn name(&self) -> &str {
    self.name
  }
}
