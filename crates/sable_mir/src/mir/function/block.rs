use crate::mir::instruction::Instruction;

#[derive(Debug)]
pub struct MirBlock<'s> {
  name: &'s str,
  instruction: Vec<Instruction<'s>>,
}

impl<'s> MirBlock<'s> {
  pub fn new(name: &'s str) -> Self {
    MirBlock {
      name,
      instruction: vec![],
    }
  }

  pub fn name(&self) -> &str {
    self.name
  }

  pub fn instructions(&self) -> &[Instruction<'s>] {
    &self.instruction
  }

  pub fn add_instruction(&mut self, instruction: Instruction<'s>) -> usize {
    self.instruction.push(instruction);
    self.instruction.len() - 1
  }
}
