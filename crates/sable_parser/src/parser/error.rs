use ariadne::Report;

pub mod unexpected_token;

pub enum ParserError<'s> {
  UnexpectedToken(unexpected_token::UnexpectedTokenError<'s>),
}

impl<'s> ParserError<'s> {
  pub fn report(&self) -> Report {
    match self {
      ParserError::UnexpectedToken(err) => err.report(),
    }
  }
}
