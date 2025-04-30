pub mod alloca;
pub mod binary;
pub mod load;
pub mod ret;
pub mod store;
pub mod call;

pub use alloca::AllocaInst;
pub use binary::AddInst;
pub use binary::DivInst;
pub use binary::MulInst;
pub use binary::SubInst;
pub use load::LoadInst;
pub use ret::ReturnInst;
pub use store::StoreInst;
pub use call::CallInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
  Alloca(AllocaInst),
  Store(StoreInst),
  Load(LoadInst),
  Add(AddInst),
  Sub(SubInst),
  Mul(MulInst),
  Div(DivInst),
  Return(ReturnInst),
  Call(CallInst),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirInstId(pub usize);
