use crate::mir::value::MirValue;

#[derive(Debug, Clone, PartialEq)]
pub struct AddInst {
  pub lhs: MirValue,
  pub rhs: MirValue,
}

impl AddInst {
  pub fn new(lhs: MirValue, rhs: MirValue) -> Self {
    AddInst { lhs, rhs }
  }
  pub fn lhs(&self) -> &MirValue {
    &self.lhs
  }
  pub fn rhs(&self) -> &MirValue {
    &self.rhs
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubInst {
  pub lhs: MirValue,
  pub rhs: MirValue,
}

impl SubInst {
  pub fn new(lhs: MirValue, rhs: MirValue) -> Self {
    SubInst { lhs, rhs }
  }
  pub fn lhs(&self) -> &MirValue {
    &self.lhs
  }
  pub fn rhs(&self) -> &MirValue {
    &self.rhs
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MulInst {
  pub lhs: MirValue,
  pub rhs: MirValue,
}

impl MulInst {
  pub fn new(lhs: MirValue, rhs: MirValue) -> Self {
    MulInst { lhs, rhs }
  }
  pub fn lhs(&self) -> &MirValue {
    &self.lhs
  }
  pub fn rhs(&self) -> &MirValue {
    &self.rhs
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DivInst {
  pub lhs: MirValue,
  pub rhs: MirValue,
}

impl DivInst {
  pub fn new(lhs: MirValue, rhs: MirValue) -> Self {
    DivInst { lhs, rhs }
  }
  pub fn lhs(&self) -> &MirValue {
    &self.lhs
  }
  pub fn rhs(&self) -> &MirValue {
    &self.rhs
  }
}