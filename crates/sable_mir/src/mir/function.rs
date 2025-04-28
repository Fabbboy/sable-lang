use std::{
  cell::{RefCell, RefMut},
  rc::Rc,
};

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
  blocks: Vec<Rc<RefCell<MirBlock<'s>>>>,
  arguments: SmallVec<[MirParam<'s>; MAX_ARGUMENTS]>,
}

impl<'s> MirFunction<'s> {
  pub fn new<'f>(mut module: RefMut<'f, MirModule<'s>>, name: &'s str, ret_type: ValType) -> Rc<RefCell<Self>> {
    let function = MirFunction {
      name,
      ret_type,
      blocks: Vec::new(),
      arguments: SmallVec::new(),
    };

    module.add_function(function)
  }

  pub fn add_block(&mut self, block: MirBlock<'s>) -> Rc<RefCell<MirBlock<'s>>> {
    self.blocks.push(Rc::new(RefCell::new(block)));
    self.blocks.last().unwrap().clone()
  }

  pub fn add_argument(&mut self, argument: MirParam<'s>) {
    self.arguments.push(argument);
  }

  pub fn get_blocks(&self) -> &[Rc<RefCell<MirBlock<'s>>>] {
    &self.blocks
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }

  pub fn get_ret_type(&self) -> &ValType {
    &self.ret_type
  }
}
