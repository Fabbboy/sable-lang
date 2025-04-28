use crate::scope::{NamendValue, Scope};

pub struct Resolver<'s> {
  scopes: Vec<Scope<'s>>,
}

impl<'s> Resolver<'s> {
  pub fn new() -> Self {
    Self {
      scopes: vec![Scope::new()],
    }
  }

  pub fn define_var(&mut self, name: &'s str, value: NamendValue<'s>) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.add_variable(name, value);
    }
  }

  pub fn resolve_var(&self, name: &'s str) -> Option<&NamendValue<'s>> {
    for scope in self.scopes.iter().rev() {
      if let Some(value) = scope.get_variable(name) {
        return Some(value);
      }
    }
    None
  }

  pub fn enter_scope(&mut self) {
    self.scopes.push(Scope::new());
  }

  pub fn exit_scope(&mut self) {
    self.scopes.pop();
  }
}
