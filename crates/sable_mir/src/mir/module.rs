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

  pub fn get_functions_mut(&mut self) -> &mut Vec<MirFunction<'s>> {
    &mut self.functions
  }

  pub fn get_functions(&self) -> &[MirFunction<'s>] {
    &self.functions
  }

  pub fn get_function(&self, idx: usize) -> Option<&MirFunction<'s>> {
    self.functions.get(idx)
  }

  pub fn get_function_mut(&mut self, idx: usize) -> Option<&mut MirFunction<'s>> {
    self.functions.get_mut(idx)
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }
}
