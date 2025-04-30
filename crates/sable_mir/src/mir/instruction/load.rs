use sable_parser::info::ValType;


use super::MirInstId;

#[derive(Debug, Clone, PartialEq)]
pub struct LoadInst {
   by: ValType,
   from: MirInstId,
}

impl LoadInst {
   pub fn new(by: ValType, from: MirInstId) -> Self {
      LoadInst { by, from }
   }

   pub fn by(&self) -> ValType {
      self.by.clone()
   }

   pub fn from(&self) -> &MirInstId {
      &self.from
   }
}