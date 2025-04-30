use block::{MirBlock, MirBlockId};

use super::instruction::{Instruction, MirInstId};

pub mod block;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirFunctionId(pub usize);

#[derive(Debug)]
pub struct MirFunction<'ctx> {
  name: &'ctx str,
  instructions: Vec<Instruction>,
  blocks: Vec<MirBlock<'ctx>>,
}

impl<'ctx> MirFunction<'ctx> {
  pub fn new(name: &'ctx str) -> Self {
    Self {
      name,
      instructions: Vec::new(),
      blocks: Vec::new(),
    }
  }

  pub fn name(&self) -> &'ctx str {
    self.name
  }

  pub fn add_inst(&mut self, inst: Instruction) -> MirInstId {
    let id = MirInstId(self.instructions.len());
    self.instructions.push(inst);
    id
  }

  pub fn add_block(&mut self, block: MirBlock<'ctx>) -> MirBlockId {
    let id = MirBlockId(self.blocks.len());
    self.blocks.push(block);
    id
  }

  pub fn get_insts(&self, blk: MirBlockId) -> &[Instruction] {
    let start = blk.0;
    let end = if blk.0 + 1 < self.blocks.len() {
      self.blocks[blk.0 + 1].range().start
    } else {
      self.instructions.len()
    };
    &self.instructions[start..end]
  }

  pub fn get_last_blk(&self) -> Option<MirBlockId> {
    if self.blocks.is_empty() {
      None
    } else {
      Some(MirBlockId(self.blocks.len() - 1))
    }
  }

  pub fn get_block(&self, id: MirBlockId) -> Option<&MirBlock<'ctx>> {
    self.blocks.get(id.0)
  }
}
