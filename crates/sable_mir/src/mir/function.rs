use std::cell::{Ref, RefCell, RefMut};

pub mod block;
pub use block::MirBlock;

#[derive(Debug)]
pub struct MirFunction<'s> {
  name: &'s str,
  blocks: Vec<RefCell<MirBlock<'s>>>,
}

impl<'s> MirFunction<'s> {
  pub fn new<'m>(name: &'s str) -> Self {
    MirFunction {
      name,
      blocks: vec![],
    }
  }

  pub fn add_block(&mut self, block: MirBlock<'s>) -> usize {
    let block = RefCell::new(block);
    self.blocks.push(block);
    self.blocks.len() - 1
  }

  pub fn name(&self) -> &str {
    self.name
  }

  pub fn get_block(&self, index: usize) -> Option<Ref<MirBlock<'s>>> {
    self.blocks.get(index).map(|cell| cell.borrow())
  }

  pub fn get_block_mut(&mut self, index: usize) -> Option<RefMut<MirBlock<'s>>> {
    self.blocks.get_mut(index).map(|cell| cell.borrow_mut())
  }
}
