use std::ops::Range;

use crate::mir::instruction::MirInstId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirBlockId(pub usize);

#[derive(Debug)]
pub struct MirBlock<'ctx> {
  name: &'ctx str,
  range: Range<usize>,
}

impl<'ctx> MirBlock<'ctx> {
  pub fn new(name: &'ctx str, start: MirInstId) -> Self {
    let rng = start.0..start.0;
    Self { name, range: rng }
  }

  pub fn expand(&mut self, inst: MirInstId) {
    if self.range.end < inst.0 {
      self.range.end = inst.0;
    }
  }

  pub fn name(&self) -> &'ctx str {
    self.name
  }

  pub fn range(&self) -> Range<usize> {
    self.range.clone()
  }
}
