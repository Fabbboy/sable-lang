use crate::mir::{function::MirFunctionId, value::MirValue};

#[derive(Debug, Clone, PartialEq)]
pub struct CallInst {
  callee: MirFunctionId,
  args: Vec<MirValue>,
}

impl CallInst {
  pub fn new(callee: MirFunctionId, args: Vec<MirValue>) -> Self {
    Self { callee, args }
  }

  pub fn callee(&self) -> MirFunctionId {
    self.callee
  }

  pub fn args(&self) -> &[MirValue] {
    &self.args
  }
}
