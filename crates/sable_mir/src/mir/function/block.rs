use crate::mir::instruction::Instruction;

use super::MirFunction;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirBlock<'s> {
  name: &'s str,
  instructions: Vec<Instruction>,
}

impl<'s> MirBlock<'s> {
  pub fn new<'b>(f: &'b mut MirFunction<'s>, name: &'s str) -> usize {
    let block = MirBlock {
      name,
      instructions: Vec::new(),
    };

    f.add_block(block);
    f.get_blocks_mut().len() - 1
  }

  pub fn add_instruction(&mut self, instruction: Instruction) {
    self.instructions.push(instruction);
  }

  pub fn get_instructions(&self) -> &[Instruction] {
    &self.instructions
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }
}
