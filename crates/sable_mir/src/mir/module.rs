use super::function::{MirFunction, MirFunctionId};

#[derive(Debug)]
pub struct MirModule<'ctx> {
  name: &'ctx str,
  funcs: Vec<MirFunction<'ctx>>,
}

impl<'ctx> MirModule<'ctx> {
  pub fn new(name: &'ctx str) -> Self {
    Self {
      name,
      funcs: Vec::new(),
    }
  }

  pub fn name(&self) -> &'ctx str {
    self.name
  }

  pub fn add_func(&mut self, func: MirFunction<'ctx>) -> MirFunctionId {
    let id = MirFunctionId(self.funcs.len());
    self.funcs.push(func);
    id
  }

  pub fn get_funcs(&self) -> &[MirFunction<'ctx>] {
    &self.funcs
  }

  pub fn get_func(&self, id: MirFunctionId) -> Option<&MirFunction<'ctx>> {
    self.funcs.get(id.0)
  }

  pub fn get_func_mut(&mut self, id: MirFunctionId) -> Option<&mut MirFunction<'ctx>> {
    self.funcs.get_mut(id.0)
  }
}
