use std::cell::RefCell;

pub mod block;
pub use block::MirBlock;

use super::module::MirModule;

#[derive(Debug)]
pub struct MirFunction<'s> {
  name: &'s str,
  blocks: Vec<RefCell<MirBlock<'s>>>,
}

impl<'s> MirFunction<'s> {
  pub fn new<'m>(module: &'m mut MirModule<'s>, name: &'s str) -> usize {
    let func = MirFunction {
      name,
      blocks: vec![],
    };

    module.add_func(func)
  }

  pub fn add_block(&mut self, block: MirBlock<'s>) -> usize {
    let block = RefCell::new(block);
    self.blocks.push(block);
    self.blocks.len() - 1
  }

  pub fn name(&self) -> &str {
    self.name
  }
}
