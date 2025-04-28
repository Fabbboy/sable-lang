use super::function::MirFunction;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MirModule<'s> {
  name: &'s str,
  functions: Vec<MirFunction<'s>>,
}

impl<'s> MirModule<'s> {
  pub fn new(name: &'s str) -> Self {
    Self {
      name,
      functions: Vec::new(),
    }
  }

  pub fn add_function(&mut self, function: MirFunction<'s>) {
    self.functions.push(function);
  }

  pub fn get_functions(&self) -> &[MirFunction<'s>] {
    &self.functions
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }
}