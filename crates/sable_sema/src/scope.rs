use std::{collections::HashMap, rc::Rc};

use sable_parser::{ast::function::Function, position::Position};

pub enum NamendValue {
  LetStmt(usize),
}

impl NamendValue {
  pub fn get_pos<'s>(&self, f: Rc<Function<'s>>) -> Position {
    match self {
      NamendValue::LetStmt(idx) => {
        let let_stmt = f.get_body().get_at(*idx).unwrap();
        let_stmt.get_pos().clone()
      }
    }
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
