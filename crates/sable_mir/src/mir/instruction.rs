pub mod assign_inst;
pub mod define_inst;

pub use assign_inst::AssignInst;
pub use define_inst::DefineInst;

#[derive(Debug)]
pub enum Instruction<'s> {
  Nop,
  Define(define_inst::DefineInst<'s>),
  Assign(assign_inst::AssignInst<'s>),
}
