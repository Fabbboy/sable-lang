pub mod constant;

#[derive(Debug, Clone)]
pub enum Value<'s> {
  Constant(constant::Constant),
  Instruction(usize),
  Str(&'s str),
}
