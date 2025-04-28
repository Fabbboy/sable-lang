use std::{cell::RefCell, rc::Rc};

use crate::mir::function::MirBlock;

pub struct Builder<'s> {
  active_block: Option<Rc<RefCell<MirBlock<'s>>>>,
}

impl<'s> Builder<'s> {
  pub fn new() -> Self {
    Self { active_block: None }
  }

  pub fn set_active_block(&mut self, block: Rc<RefCell<MirBlock<'s>>>) {
    self.active_block = Some(block.clone());
  }
}
