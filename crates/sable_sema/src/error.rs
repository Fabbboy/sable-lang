use sable_parser::parser::error::ParseErrReport;

pub mod expr_errs;
pub mod func_already_defined;
pub mod var_redeclared;

pub use expr_errs::SemaExprError;
pub use func_already_defined::FunctionAlreadyDefined;
pub use var_redeclared::VariableRedeclared;

pub enum AnalyzerError<'s> {
  FunctionAlreadyDefined(func_already_defined::FunctionAlreadyDefined<'s>),
  VariableRedeclared(var_redeclared::VariableRedeclared<'s>),
  ExprError(expr_errs::SemaExprError<'s>),
}

impl<'s> AnalyzerError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    match self {
      AnalyzerError::FunctionAlreadyDefined(err) => err.report(filename),
      AnalyzerError::VariableRedeclared(err) => err.report(filename),
      AnalyzerError::ExprError(err) => err.report(filename),
    }
  }
}
