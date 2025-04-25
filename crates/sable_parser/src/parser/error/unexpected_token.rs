use ariadne::{Label, Report, ReportKind};
use smallvec::SmallVec;

use crate::lexer::token::{Token, TokenType};

use super::ParseErrReport;

pub const MAX_EXPECTED: usize = 8;

#[derive(Debug, Clone)]
pub struct UnexpectedTokenError<'s> {
  expected: SmallVec<[TokenType; MAX_EXPECTED]>,
  found: Token<'s>,
}

impl<'s> UnexpectedTokenError<'s> {
  pub fn new(expected: SmallVec<[TokenType; MAX_EXPECTED]>, found: Token<'s>) -> Self {
    Self { expected, found }
  }

  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    let short = (filename, self.found.pos.range.clone());
    let one_of_msg = if self.expected.len() == 1 {
      format!("expected: {:?}", self.expected[0])
    } else {
      format!(
        "expected one of: {}",
        self
          .expected
          .iter()
          .map(|t| format!("{:?}", t))
          .collect::<Vec<_>>()
          .join(", ")
      )
    };

    Report::build(ReportKind::Error, short.clone())
      .with_label(
        Label::new(short)
          .with_message(format!("unexpected token: {:?}", self.found.token_type))
          .with_color(ariadne::Color::Red),
      )
      .with_note(one_of_msg)
      .finish()
  }
}

#[cfg(test)]
mod tests {
  use ariadne::Source;
  use smallvec::smallvec;

  use super::*;
  use crate::lexer::{lexer::Lexer, token::TokenType};

  const FILENAME: &str = "test.sbl";
  const SOURCE: &str = r#"let 2 = 123"#;

  #[test]
  fn test_unexpected_token_error() {
    let mut lexer = Lexer::new(SOURCE);
    lexer.lex();
    let unexpected = lexer.lex();
    let expected = smallvec![TokenType::Identifier];

    let err = UnexpectedTokenError::new(expected.clone(), unexpected.clone());
    let report = err.report(FILENAME);
    report.print((FILENAME, Source::from(SOURCE))).unwrap();
  }
}
