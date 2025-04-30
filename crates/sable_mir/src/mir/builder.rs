use std::{cell::RefCell, rc::Rc};

use super::{
  function::{
    MirFunctionId,
    block::{MirBlock, MirBlockId},
  },
  module::MirModule,
};

pub struct Builder<'ctx> {
  selected: Option<MirBlockId>,
  selected_fn: MirFunctionId,
  module: Rc<RefCell<MirModule<'ctx>>>,
}

impl<'ctx> Builder<'ctx> {
  pub fn new(module: Rc<RefCell<MirModule<'ctx>>>, selected_fn: MirFunctionId) -> Self {
    Self {
      selected: None,
      selected_fn,
      module,
    }
  }

  pub fn set_selected(&mut self, block_id: MirBlockId) {
    self.selected = Some(block_id);
  }
}
