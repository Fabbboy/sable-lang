use ariadne::Report;
use smallvec::SmallVec;

use crate::lexer::token::{Token, TokenType};

const MAX_EXPECTED: usize = 8;

pub struct UnexpectedTokenError<'s> {
  pub expected: SmallVec<[TokenType; MAX_EXPECTED]>,
  pub found: Token<'s>,
}

impl<'s> UnexpectedTokenError<'s> {
  pub fn new(expected: SmallVec<[TokenType; MAX_EXPECTED]>, found: Token<'s>) -> Self {
    Self { expected, found }
  }

  pub fn report(&self) -> Report {
    todo!()
  }
}
