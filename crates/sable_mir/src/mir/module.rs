pub struct MirModule<'ctx> {
  name: &'ctx str,
}

impl<'ctx> MirModule<'ctx> {
  pub fn new(name: &'ctx str) -> Self {
    Self { name }
  }

  pub fn name(&self) -> &'ctx str {
    self.name
  }
}
