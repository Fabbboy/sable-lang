pub mod alloca;
pub mod store;

pub use alloca::AllocaInst;
pub use store::StoreInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
  Alloca(AllocaInst),
  Store(StoreInst),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirInstId(pub usize);
