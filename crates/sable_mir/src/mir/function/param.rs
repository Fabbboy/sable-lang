use sable_parser::info::ValType;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirParam<'s> {
  name: &'s str,
  type_: ValType,
}

impl<'s> MirParam<'s> {
  pub fn new(name: &'s str, type_: ValType) -> Self {
    Self { name, type_ }
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }

  pub fn get_type(&self) -> &ValType {
    &self.type_
  }
}
