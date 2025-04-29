use crate::mir::value::Value;

#[derive(Debug, Clone)]
pub enum BinaryInst<'s> {
  Add(Value<'s>, Value<'s>),
  Sub(Value<'s>, Value<'s>),
  Mul(Value<'s>, Value<'s>),
  Div(Value<'s>, Value<'s>),
}

impl<'s> BinaryInst<'s> {
  pub fn add(lhs: Value<'s>, rhs: Value<'s>) -> Self {
    BinaryInst::Add(lhs, rhs)
  }

  pub fn sub(lhs: Value<'s>, rhs: Value<'s>) -> Self {
    BinaryInst::Sub(lhs, rhs)
  }

  pub fn mul(lhs: Value<'s>, rhs: Value<'s>) -> Self {
    BinaryInst::Mul(lhs, rhs)
  }

  pub fn div(lhs: Value<'s>, rhs: Value<'s>) -> Self {
    BinaryInst::Div(lhs, rhs)
  }
}
