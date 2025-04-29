use std::cell::RefMut;

use sable_parser::info::ValType;

use crate::mir::instruction::AssignInst;

use super::{
  error::MirError,
  function::{MirBlock, MirFunction},
  instruction::{DefineInst, Instruction},
  value::Value,
};

pub struct MirBuilder<'s, 'b> {
  func: &'b mut MirFunction<'s>,
  active_block: Option<usize>,
}

impl<'s, 'b> MirBuilder<'s, 'b> {
  pub fn new(func: &'b mut MirFunction<'s>) -> Self {
    MirBuilder {
      func,
      active_block: None,
    }
  }

  fn get_active_block(&mut self) -> Result<RefMut<MirBlock<'s>>, MirError<'s>> {
    if let Some(block_index) = self.active_block {
      if let Some(block) = self.func.get_block_mut(block_index) {
        Ok(block)
      } else {
        Err(MirError::BlockNotFound(block_index))
      }
    } else {
      Err(MirError::NoActiveBlock)
    }
  }

  pub fn create_define(
    &mut self,
    name: &'s str,
    type_: ValType,
    value: Value<'s>,
  ) -> Result<Value<'s>, MirError<'s>> {
    let mut block = self.get_active_block()?;
    let define = DefineInst::new(name, type_, value);
    let inst = Instruction::Define(define);
    let idx = block.add_instruction(inst);
    Ok(Value::Instruction(idx))
  }

  pub fn create_assign(&mut self, dest: &'s str, src: Value<'s>) -> Result<Value<'s>, MirError<'s>> {
    let mut block = self.get_active_block()?;
    let assign = Instruction::Assign(AssignInst::new(dest, src));
    let idx = block.add_instruction(assign);
    Ok(Value::Instruction(idx))
  }

  pub fn set_insert(&mut self, block: usize) -> Result<(), MirError<'s>> {
    if self.func.get_block(block).is_none() {
      return Err(MirError::BlockNotFound(block));
    }
    self.active_block = Some(block);
    Ok(())
  }
}
