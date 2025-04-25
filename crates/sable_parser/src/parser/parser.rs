use std::rc::Rc;

use smallvec::{SmallVec, smallvec};

use crate::{
  ast::ast::AST,
  lexer::{lexer::Lexer, token::TokenType},
};

use super::error::{
  ParserError,
  unexpected_token::{MAX_EXPECTED, UnexpectedTokenError},
};

pub type PRes<'s, T> = Result<T, ParserError<'s>>;

pub struct Parser<'p, 's> {
  lexer: &'p mut Lexer<'s>,
  ast: Rc<AST>,
  errs: Vec<ParserError<'s>>,
}

impl<'p, 's> Parser<'p, 's> {
  pub fn new(lexer: &'p mut Lexer<'s>) -> Self {
    Parser {
      lexer,
      ast: Rc::new(AST::new()),
      errs: Vec::new(),
    }
  }

  fn next(&mut self, expected: SmallVec<[TokenType; MAX_EXPECTED]>) -> PRes<TokenType> {
    let token = self.lexer.lex();
    if token.token_type == TokenType::Err {
      let err = UnexpectedTokenError::new(expected, token);
      return Err(ParserError::UnexpectedToken(err));
    }

    for expected_token in expected.iter() {
      if token.token_type == *expected_token {
        return Ok(token.token_type);
      }
    }

    let err = UnexpectedTokenError::new(expected, token);
    Err(ParserError::UnexpectedToken(err))
  }

  pub fn parse(&mut self) -> Result<Rc<AST>, &Vec<ParserError<'s>>> {
    if !self.errs.is_empty() {
      return Err(&self.errs);
    }

    Ok(self.ast.clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lexer::token::TokenType;

  #[test]
  fn test_parser() {
    let source = "abc";
    let mut lexer = Lexer::new(source);
    let parser = Parser::new(&mut lexer);

    let token = parser.lexer.lex();
    assert_eq!(token.token_type, TokenType::Identifier);
    assert_eq!(token.lexeme, "abc");
  }

  #[test]
  fn test_err_or_ast() {
    let source = "abc";
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(&mut lexer);

    let result = parser.parse();
    assert!(result.is_ok());
  }
}
