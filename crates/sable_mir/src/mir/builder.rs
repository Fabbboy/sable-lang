use std::{cell::RefCell, rc::Rc};

use sable_parser::info::ValType;

use crate::lowering::NamendPlace;

use super::{
  function::{block::MirBlockId, MirFunctionId},
  instruction::{
    alloca::AllocaInst, binary::{DivInst, SubInst}, ret::ReturnInst, AddInst, Instruction, LoadInst, MirInstId, MulInst, StoreInst
  },
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

  pub fn build_store(&mut self, dest: MirInstId, value: MirValue) {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Store(StoreInst::new(dest, value));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
  }

  pub fn build_load(&mut self, by: ValType, from: NamendPlace) -> MirInstId {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Load(LoadInst::new(by, from));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
    inst_id
  }

  pub fn build_add(&mut self, lhs: MirValue, rhs: MirValue) -> MirInstId {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Add(AddInst::new(lhs, rhs));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
    inst_id
  }

  pub fn build_sub(&mut self, lhs: MirValue, rhs: MirValue) -> MirInstId {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Sub(SubInst::new(lhs, rhs));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
    inst_id
  }

  pub fn build_mul(&mut self, lhs: MirValue, rhs: MirValue) -> MirInstId {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Mul(MulInst::new(lhs, rhs));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
    inst_id
  }

  pub fn build_div(&mut self, lhs: MirValue, rhs: MirValue) -> MirInstId {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Div(DivInst::new(lhs, rhs));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
    inst_id
  }

  pub fn build_return(&mut self, type_: ValType, value: MirValue) {
    let mut module = self.module.borrow_mut();
    let func = module.get_func_mut(self.selected_fn).unwrap();
    let inst = Instruction::Return(ReturnInst::new(value, type_));
    let inst_id = func.add_inst(inst);
    let block_id = self.selected.unwrap();
    func.get_block_mut(block_id).unwrap().expand(inst_id);
  }
}
