use sable_parser::info::ValType;

use crate::mir::value::Value;

#[derive(Debug)]
pub struct DefineInst<'s> {
  name: &'s str,
  type_: ValType,
  value: Value,
}

impl<'a> DefineInst<'a> {
  pub fn new(name: &'a str, type_: ValType, value: Value) -> Self {
    Self { name, type_, value }
  }

  pub fn name(&self) -> &str {
    self.name
  }

  pub fn type_(&self) -> ValType {
    self.type_.clone()
  }

  pub fn value(&self) -> &Value {
    &self.value
  }
}
