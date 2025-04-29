use super::instruction::Instruction;

pub mod constant;

#[derive(Debug, Clone)]
pub enum Value<'s> {
  Constant(constant::Constant),
  Instruction(Box<Instruction<'s>>),
  Str(&'s str),
}
