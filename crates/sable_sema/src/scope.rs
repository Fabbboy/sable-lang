use std::collections::HashMap;

use sable_parser::ast::statement::LetStatement;

pub enum NamendValue<'s> {
  LetStatement(&'s LetStatement<'s>),
}

pub struct Scope<'s> {
  variables: HashMap<&'s str, NamendValue<'s>>,
}

impl<'s> Scope<'s> {
  pub fn new() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }

  pub fn add_variable(&mut self, name: &'s str, value: NamendValue<'s>) {
    self.variables.insert(name, value);
  }

  pub fn get_variable(&self, name: &'s str) -> Option<&NamendValue<'s>> {
    self.variables.get(name)
  }
}
