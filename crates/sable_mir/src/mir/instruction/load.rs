use sable_parser::info::ValType;

use crate::lowering::NamendPlace;


#[derive(Debug, Clone, PartialEq)]
pub struct LoadInst {
  by: ValType,
  from: NamendPlace,
}

impl LoadInst {
  pub fn new(by: ValType, from: NamendPlace) -> Self {
    LoadInst { by, from }
  }

  pub fn by(&self) -> ValType {
    self.by.clone()
  }

  pub fn from(&self) -> &NamendPlace {
    &self.from
  }
}
