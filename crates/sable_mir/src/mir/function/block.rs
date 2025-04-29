use std::cell::RefMut;

use crate::mir::instruction::Instruction;

use super::MirFunction;

#[derive(Debug)]
pub struct MirBlock<'s> {
  name: &'s str,
  instruction: Vec<Instruction>,
}

impl<'s> MirBlock<'s> {
  pub fn new(mut func: RefMut<MirFunction<'s>>, name: &'s str) -> usize {
    let block = MirBlock {
      name,
      instruction: vec![],
    };
    func.add_block(block)
  }

  pub fn name(&self) -> &str {
    self.name
  }

  pub fn instructions(&self) -> &[Instruction] {
    &self.instruction
  }

  pub fn add_instruction(&mut self, instruction: Instruction) -> usize {
    self.instruction.push(instruction);
    self.instruction.len() - 1
  }
}
