use sable_parser::info::ValType;

#[derive(Debug, Clone, PartialEq)]
pub struct AllocaInst {
  type_: ValType,
}

impl AllocaInst {
  pub fn new(type_: ValType) -> Self {
    AllocaInst { type_ }
  }

  pub fn type_(&self) -> ValType {
    self.type_.clone()
  }
}
