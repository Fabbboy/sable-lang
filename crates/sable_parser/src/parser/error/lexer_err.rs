use ariadne::{Label, Report, ReportKind};

use crate::lexer::token::Token;

use super::ParseErrReport;

#[derive(Debug, Clone)]
pub struct LexerError<'s> {
  token: Token<'s>,
}

impl<'s> LexerError<'s> {
  pub fn new(token: Token<'s>) -> Self {
    Self { token }
  }

  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    let short = (filename, self.token.pos.range.clone());
    Report::build(ReportKind::Error, short.clone())
      .with_label(Label::new(short).with_message("lexer failed to tokenize this")).finish()
  }
}
