use smallvec::SmallVec;

pub mod block;
pub mod param;

pub use block::MirBlock;
pub use param::MirParam;

const MAX_ARGUMENTS: usize = 4;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirFunction<'s> {
  name: &'s str,
  blocks: Vec<MirBlock<'s>>,
  arguments: SmallVec<[MirParam<'s>; MAX_ARGUMENTS]>,
}

impl<'s> MirFunction<'s> {
  pub fn new(name: &'s str) -> Self {
    Self {
      name,
      blocks: Vec::new(),
      arguments: SmallVec::new(),
    }
  }

  pub fn add_block(&mut self, block: MirBlock<'s>) {
    self.blocks.push(block);
  }

  pub fn add_argument(&mut self, argument: MirParam<'s>) {
    self.arguments.push(argument);
  }

  pub fn get_blocks(&self) -> &[MirBlock<'s>] {
    &self.blocks
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }
}
