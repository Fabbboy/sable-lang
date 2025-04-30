pub mod alloca;
pub mod store;
pub mod load;
pub mod binary;
pub mod ret;

pub use alloca::AllocaInst;
pub use store::StoreInst;
pub use load::LoadInst;
pub use binary::AddInst;
pub use binary::SubInst;
pub use binary::MulInst;
pub use binary::DivInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
  Alloca(AllocaInst),
  Store(StoreInst),
  Load(LoadInst),
  Add(AddInst),
  Sub(SubInst),
  Mul(MulInst),
  Div(DivInst),
  Return(ret::ReturnInst),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirInstId(pub usize);
