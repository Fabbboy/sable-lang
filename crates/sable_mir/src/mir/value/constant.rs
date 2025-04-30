use sable_parser::info::ValType;

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
  Null,
  IntValue(ValType, u64),
  FloatValue(ValType, f64),
}
