use std::collections::HashMap;

use sable_parser::{info::ValType, position::Position};

pub struct NamendValue {
  val_type: ValType,
  position: Position,
}

impl NamendValue {
  pub fn new(val_type: ValType, position: Position) -> Self {
    Self { val_type, position }
  }

  pub fn get_pos(&self) -> &Position {
    &self.position
  }

  pub fn get_type(&self) -> &ValType {
    &self.val_type
  }
}
pub struct Scope<'s> {
  variables: HashMap<&'s str, NamendValue>,
}

impl<'s> Scope<'s> {
  pub fn new() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }

  pub fn add_variable(&mut self, name: &'s str, value: NamendValue) {
    self.variables.insert(name, value);
  }

  pub fn get_variable(&self, name: &'s str) -> Option<&NamendValue> {
    self.variables.get(name)
  }

  pub fn is_declared(&self, name: &'s str) -> bool {
    self.variables.contains_key(name)
  }
}
