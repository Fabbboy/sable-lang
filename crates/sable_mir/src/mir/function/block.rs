use std::{cell::{RefCell, RefMut}, rc::Rc};


use crate::mir::instruction::Instruction;

use super::MirFunction;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirBlock<'s> {
  name: &'s str,
  instructions: Vec<Instruction>,
}

impl<'s> MirBlock<'s> {
  pub fn new<'b>(mut f: RefMut<'b, MirFunction<'s>>, name: &'s str) -> Rc<RefCell<Self>> {
    let block = MirBlock {
      name,
      instructions: Vec::new(),
    };

    f.add_block(block)
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
