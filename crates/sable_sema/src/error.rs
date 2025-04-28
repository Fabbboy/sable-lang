use sable_parser::parser::error::ParseErrReport;

pub mod func_already_defined;
pub mod var_redeclared;

pub enum AnalyzerError<'s> {
  FunctionAlreadyDefined(func_already_defined::FunctionAlreadyDefined<'s>),
  VariableRedeclared(var_redeclared::VariableRedeclared<'s>),
}

impl<'s> AnalyzerError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    match self {
      AnalyzerError::FunctionAlreadyDefined(err) => err.report(filename),
      AnalyzerError::VariableRedeclared(err) => err.report(filename),
    }
  }
}
