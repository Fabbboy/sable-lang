#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ValType {
  I32,
  F32,
  Void,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum OperatorType {
  Add,
  Sub,
  Mul,
  Div,
}
