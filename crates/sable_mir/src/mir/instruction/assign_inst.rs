use crate::mir::value::Value;

#[derive(Debug, Clone)]
pub struct AssignInst<'s> {
  dest: &'s str,
  src: Value<'s>,
}

impl<'a> AssignInst<'a> {
  pub fn new(dest: &'a str, src: Value<'a>) -> Self {
    Self { dest, src }
  }

  pub fn dest(&self) -> &str {
    self.dest
  }

  pub fn src(&self) -> &Value {
    &self.src
  }
}
