use super::function::Function;

#[derive(Debug, Default)]
pub struct AST<'s> {
  functions: Vec<Function<'s>>,
}

impl<'s> AST<'s> {
  pub fn new() -> Self {
    AST {
      functions: Vec::new(),
    }
  }

  pub fn add_func(&mut self, f: Function<'s>) {
    self.functions.push(f);
  }
}
