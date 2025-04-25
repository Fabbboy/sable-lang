use std::{
  cell::{Ref, RefCell},
  rc::Rc,
};

use smallvec::{SmallVec, smallvec};

use crate::{
  ast::ast::AST,
  lexer::{
    lexer::Lexer,
    token::{Token, TokenType},
  },
};

use super::error::{
  ParserError,
  unexpected_token::{MAX_EXPECTED, UnexpectedTokenError},
};

pub type PRes<'s, T> = Result<T, ParserError<'s>>;

pub struct Parser<'s> {
  lexer: RefCell<Lexer<'s>>,
  ast: Rc<AST>,
  errs: Vec<ParserError<'s>>,
}

impl<'s> Parser<'s> {
  pub fn new(lexer: Lexer<'s>) -> Self {
    Parser {
      lexer: RefCell::new(lexer),
      ast: Rc::new(AST::new()),
      errs: Vec::new(),
    }
  }

  fn next(&self, expected: SmallVec<[TokenType; MAX_EXPECTED]>) -> Result<Token<'s>, ParserError<'s>> {
    let token = self.lexer.borrow_mut().lex();
    if token.token_type == TokenType::Err {
      let err = UnexpectedTokenError::new(expected, token);
      return Err(ParserError::UnexpectedToken(err));
    }

    for expected_token in expected.iter() {
      if token.token_type == *expected_token {
        return Ok(token);
      }
    }

    let err = UnexpectedTokenError::new(expected, token);
    Err(ParserError::UnexpectedToken(err))
  }

  pub fn parse(&mut self) -> Result<Rc<AST>, &[ParserError<'s>]> {
    loop {
      let tok = self.next(smallvec![TokenType::Func, TokenType::Eof]);
      match tok {
        Err(err) => {
          self.errs.push(err);
          break;
        }
        Ok(token) => match token.token_type {
          TokenType::Func => {}
          TokenType::Eof => break,
          _ => unreachable!(),
        },
      }
    }

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
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer);

    let token = parser.lexer.borrow_mut().lex();
    assert_eq!(token.token_type, TokenType::Identifier);
    assert_eq!(token.lexeme, "abc");
  }

  #[test]
  fn test_err_or_ast() {
    let source = "abc";
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let result = parser.parse();
    assert!(result.is_ok());
  }
}
