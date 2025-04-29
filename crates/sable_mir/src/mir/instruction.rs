use super::value::Value;

#[derive(Debug)]
pub enum Instruction {
  Nop,
  Terminator(Terminator)
}

#[derive(Debug)]
pub enum Terminator {
    Return(Value)
}