use std::cell::RefMut;

use super::{
  error::MirError,
  function::{MirBlock, MirFunction},
};

pub struct MirBuilder<'s, 'b> {
  func: &'b mut MirFunction<'s>,
  active_block: Option<usize>,
}

impl<'s, 'b> MirBuilder<'s, 'b> {
  pub fn new(func: &'b mut MirFunction<'s>) -> Self {
    MirBuilder {
      func,
      active_block: None,
    }
  }

  fn get_active_block(&mut self) -> Result<RefMut<MirBlock<'s>>, MirError> {
    if let Some(block_index) = self.active_block {
      if let Some(block) = self.func.get_block_mut(block_index) {
        Ok(block)
      } else {
        Err(MirError::BlockNotFound(block_index))
      }
    } else {
      Err(MirError::NoActiveBlock)
    }
  }

  pub fn set_insert(&mut self, block: usize) -> Result<(), MirError> {
    if self.func.get_block(block).is_none() {
      return Err(MirError::BlockNotFound(block));
    }
    self.active_block = Some(block);
    Ok(())
  }
}
