use block::MirBlock;

pub mod block;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirFunction<'s> {
  name: &'s str,
  blocks: Vec<MirBlock<'s>>,
}

impl<'s> MirFunction<'s> {
  pub fn new(name: &'s str) -> Self {
    Self {
      name,
      blocks: Vec::new(),
    }
  }

  pub fn add_block(&mut self, block: MirBlock<'s>) {
    self.blocks.push(block);
  }

  pub fn get_blocks(&self) -> &[MirBlock<'s>] {
    &self.blocks
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }
}
