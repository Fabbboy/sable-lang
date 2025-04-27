use std::marker::PhantomData;

use sable_parser::parser::error::ParseErrReport;

pub enum AnalyzerError<'s> {
  _P(PhantomData<&'s ()>),
}

impl<'s> AnalyzerError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    todo!()
  }
}
