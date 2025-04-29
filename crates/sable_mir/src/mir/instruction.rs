pub mod define_inst;

pub use define_inst::DefineInst;

#[derive(Debug)]
pub enum Instruction<'s> {
  Nop,
  Define(define_inst::DefineInst<'s>),
}
