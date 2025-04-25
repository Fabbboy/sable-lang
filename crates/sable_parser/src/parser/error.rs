use std::ops::Range;

use ariadne::Report;

pub mod unexpected_token;
pub mod lexer_err;

pub type ParseErrReport<'s> = Report<'s, (&'s str, Range<usize>)>;

#[derive(Debug)]
pub enum ParserError<'s> {
  UnexpectedToken(unexpected_token::UnexpectedTokenError<'s>),
  LexerError(lexer_err::LexerError<'s>),
}

impl<'s> ParserError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    match self {
      ParserError::UnexpectedToken(err) => err.report(filename),
      ParserError::LexerError(err) => err.report(filename)
    }
  }
}
