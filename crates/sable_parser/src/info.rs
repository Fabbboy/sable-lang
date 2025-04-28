use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ValType {
  Untyped,
  I32,
  F32,
  Void,
}

impl Display for ValType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ValType::Untyped => write!(f, "untyped"),
      ValType::I32 => write!(f, "i32"),
      ValType::F32 => write!(f, "f32"),
      ValType::Void => write!(f, "void"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum OperatorType {
  Add,
  Sub,
  Mul,
  Div,
}
