use std::ops::Range;

use ariadne::Report;

pub mod unexpected_token;

pub type ParseErrReport<'s> = Report<'s, (&'s str, Range<usize>)>;

pub enum ParserError<'s> {
  UnexpectedToken(unexpected_token::UnexpectedTokenError<'s>),
}

impl<'s> ParserError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    match self {
      ParserError::UnexpectedToken(err) => err.report(filename),
    }
  }
}
