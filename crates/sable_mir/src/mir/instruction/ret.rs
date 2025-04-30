use sable_parser::info::ValType;

use crate::mir::value::MirValue;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnInst {
  ret_value: MirValue,
  type_: ValType
}

impl ReturnInst {
  pub fn new(ret_value: MirValue, type_: ValType) -> Self {
    ReturnInst { ret_value, type_ }
  }

  pub fn ret_value(&self) -> &MirValue {
    &self.ret_value
  }

  pub fn type_(&self) -> ValType {
    self.type_.clone()
  }
}
