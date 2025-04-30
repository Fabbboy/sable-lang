pub mod constant;

pub use constant::Constant;

use super::instruction::MirInstId;

#[derive(Debug, Clone, PartialEq)]
pub enum MirValue {
  Constant(constant::Constant),
  Inst(MirInstId),
}
