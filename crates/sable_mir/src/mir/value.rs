pub mod constant;

pub use constant::Constant;

#[derive(Debug, Clone, PartialEq)]
pub enum MirValue {
  Constant(constant::Constant),
}
