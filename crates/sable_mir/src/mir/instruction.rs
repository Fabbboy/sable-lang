pub mod assign_inst;
pub mod binary_inst;
pub mod define_inst;

pub use assign_inst::AssignInst;
pub use binary_inst::BinaryInst;
pub use define_inst::DefineInst;

#[derive(Debug)]
pub enum Instruction<'s> {
  Nop,
  Define(define_inst::DefineInst<'s>),
  Assign(assign_inst::AssignInst<'s>),
  Binary(binary_inst::BinaryInst<'s>),
}
