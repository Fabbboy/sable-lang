use crate::mir::value::MirValue;

use super::MirInstId;

#[derive(Debug, Clone, PartialEq)]
pub struct StoreInst {
  target: MirInstId,
  value: MirValue,
}

impl StoreInst {
  pub fn new(target: MirInstId, value: MirValue) -> Self {
    StoreInst { target, value }
  }
  pub fn target(&self) -> MirInstId {
    self.target
  }
  pub fn value(&self) -> &MirValue {
    &self.value
  }
}
