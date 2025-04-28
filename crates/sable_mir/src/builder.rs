use std::{cell::RefCell, rc::Rc};

use crate::mir::{function::MirBlock, module::MirModule};

pub struct Builder<'s> {
  module: Rc<RefCell<MirModule<'s>>>,
  active_block: Option<usize>,
}

impl<'s> Builder<'s> {
  pub fn new(module: Rc<RefCell<MirModule<'s>>>) -> Self {
    Self {
      module,
      active_block: None,
    }
  }

  fn get_active_block(&mut self) -> Option<Rc<MirBlock<'s>>> {
    if let Some(block_idx) = self.active_block {
      let module = self.module.borrow();
      let function = module.get_function(0).unwrap(); // Assuming function index 0 for now
      function.get_blocks().get(block_idx).cloned()
    } else {
      None
    }
  }

  pub fn set_active_block(&mut self, block_idx: usize) {
    self.active_block = Some(block_idx);
  }
}
