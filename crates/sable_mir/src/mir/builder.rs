use std::{cell::RefCell, rc::Rc};

use sable_parser::info::ValType;

use super::{
  function::{block::MirBlockId, MirFunctionId},
  instruction::{alloca::AllocaInst, Instruction, MirInstId, StoreInst},
  module::MirModule,
  value::MirValue,
};

pub struct Builder<'ctx> {
  selected: Option<MirBlockId>,
  selected_fn: MirFunctionId,
  module: Rc<RefCell<MirModule<'ctx>>>,
}

impl<'ctx> Builder<'ctx> {
  pub fn new(module: Rc<RefCell<MirModule<'ctx>>>, selected_fn: MirFunctionId) -> Self {
    Self {
      selected: None,
      selected_fn,
      module,
    }
  }

  pub fn set_selected(&mut self, block_id: MirBlockId) {
    self.selected = Some(block_id);
  }

  pub fn build_alloca(&mut self, type_: ValType) -> MirInstId {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = AllocaInst::new(type_);
    let inst_id = func.add_inst(Instruction::Alloca(inst));
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
    inst_id
  }

  pub fn build_store(&mut self, dest: MirInstId, value: MirValue) -> MirInstId {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Store(StoreInst::new(dest, value));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
    inst_id
  }
}
