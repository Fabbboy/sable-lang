pub mod constant;

#[derive(Debug)]
pub enum Value {
  Constant(constant::Constant),
  Instruction(usize),
}
