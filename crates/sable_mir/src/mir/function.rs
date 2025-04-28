use std::{cell::RefMut, rc::Rc};

use sable_parser::info::ValType;
use smallvec::SmallVec;

pub mod block;
pub mod param;

pub use block::MirBlock;
pub use param::MirParam;

use super::module::MirModule;

const MAX_ARGUMENTS: usize = 4;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirFunction<'s> {
  name: &'s str,
  ret_type: ValType,
  blocks: Vec<Rc<MirBlock<'s>>>,
  arguments: SmallVec<[MirParam<'s>; MAX_ARGUMENTS]>,
}

impl<'s> MirFunction<'s> {
  pub fn new<'f>(mut module: RefMut<'f, MirModule<'s>>, name: &'s str, ret_type: ValType) -> usize {
    let function = MirFunction {
      name,
      ret_type,
      blocks: Vec::new(),
      arguments: SmallVec::new(),
    };

    module.add_function(function);
    module.get_functions_mut().len() - 1
  }

  pub fn add_block(&mut self, block: MirBlock<'s>) {
    self.blocks.push(Rc::new(block));
  }

  pub fn add_argument(&mut self, argument: MirParam<'s>) {
    self.arguments.push(argument);
  }

  pub fn get_blocks(&self) -> &[Rc<MirBlock<'s>>] {
    &self.blocks
  }

  pub fn get_blocks_mut(&mut self) -> &mut [Rc<MirBlock<'s>>] {
    &mut self.blocks
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }

  pub fn get_ret_type(&self) -> &ValType {
    &self.ret_type
  }

  pub fn get_block(&self, idx: usize) -> Option<Rc<MirBlock<'s>>> {
    self.blocks.get(idx).cloned()
  }

  pub fn get_block_mut(&mut self, idx: usize) -> Option<Rc<MirBlock<'s>>> {
    self.blocks.get_mut(idx).cloned()
  }
}
