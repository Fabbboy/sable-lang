use sable_parser::info::ValType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweringError<'ctx> {
  InvalidNumericValue(&'ctx str),
  IllegalType(ValType),
  VariableNotFound(&'ctx str),
}