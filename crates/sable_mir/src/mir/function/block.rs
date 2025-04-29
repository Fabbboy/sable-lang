use std::cell::RefMut;

use crate::mir::{
  instruction::{Instruction, Terminator},
  value::{Value, constant::Constant},
};

use super::MirFunction;

#[derive(Debug)]
pub struct MirBlock<'s> {
  name: &'s str,
  instruction: Vec<Instruction>,
  terminator: Terminator,
}

impl<'s> MirBlock<'s> {
  pub fn new(mut func: RefMut<MirFunction<'s>>, name: &'s str) -> usize {
    let block = MirBlock {
      name,
      instruction: vec![],
      terminator: Terminator::Return(Value::Constant(Constant::Null)),
    };
    func.add_block(block)
  }

  pub fn name(&self) -> &str {
    self.name
  }

  pub fn instructions(&self) -> &[Instruction] {
    &self.instruction
  }

  pub fn terminator(&self) -> &Terminator {
    &self.terminator
  }

  pub fn add_instruction(&mut self, instruction: Instruction) -> usize {
    self.instruction.push(instruction);
    self.instruction.len() - 1
  }
}
