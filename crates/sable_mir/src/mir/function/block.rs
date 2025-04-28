use crate::mir::instruction::Instruction;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirBlock<'s> {
  name: &'s str,
  instructions: Vec<Instruction>,
}

impl<'s> MirBlock<'s> {
  pub fn new(name: &'s str) -> Self {
    Self {
      name,
      instructions: Vec::new(),
    }
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
