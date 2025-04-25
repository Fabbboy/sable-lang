use std::{
  cell::{Ref, RefCell},
  rc::Rc,
};

use smallvec::{SmallVec, smallvec};

use crate::{
  ast::{ast::AST, function::Function},
  lexer::{
    lexer::Lexer,
    token::{Token, TokenType},
  },
};

use super::error::{
  ParserError,
  unexpected_token::{MAX_EXPECTED, UnexpectedTokenError},
};

macro_rules! next {
  (@plain $self:expr, [$($expected:expr),+]) => {{
    match $self.next(smallvec![$($expected),+]) {
      Ok(tok) => tok,
      Err(err) => return Err(err),
    }
  }};

  (@vec $self:expr, [$($expected:expr),+]) => {{
    match $self.next(smallvec![$($expected),+]) {
      Ok(tok) => tok,
      Err(err) => return Err(smallvec![err]),
    }
  }};
}

pub struct Parser<'s> {
  lexer: &'s mut Lexer<'s>,
  ast: Rc<RefCell<AST<'s>>>,
  errs: Vec<ParserError<'s>>,
}

impl<'s> Parser<'s> {
  pub fn new(lexer: &'s mut Lexer<'s>) -> Parser<'s> {
    Parser {
      lexer,
      ast: Rc::new(RefCell::new(AST::new())),
      errs: Vec::new(),
    }
  }

  fn next(
    &mut self,
    expected: SmallVec<[TokenType; MAX_EXPECTED]>,
  ) -> Result<Token<'s>, ParserError<'s>> {
    let token = self.lexer.lex();
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

  fn sync(&mut self, expected: SmallVec<[TokenType; MAX_EXPECTED]>) {
    loop {
      let token = self.lexer.peek();
      if token.token_type == TokenType::Eof {
        break;
      }
      if expected.iter().any(|t| *t == token.token_type) {
        break;
      }
      self.lexer.lex();
    }
  }

  fn parse_function(&mut self) -> Result<Function<'s>, SmallVec<[ParserError<'s>; MAX_EXPECTED]>> {
    let type_ = next!(@vec self, [TokenType::Type]);
    let mut ty_pos = type_.pos;
    let name = next!(@vec self, [TokenType::Identifier]);
    next!(@vec self, [TokenType::Paren(true)]);
    next!(@vec self, [TokenType::Paren(false)]);
    next!(@vec self, [TokenType::Brace(true)]);
    next!(@vec self, [TokenType::Brace(false)]);

    ty_pos.range.end = name.pos.range.end;
    Ok(Function::new(name.lexeme, ty_pos))
  }

  pub fn parse(&mut self) -> Result<Ref<AST>, &[ParserError<'s>]> {
    loop {
      let tok = self.next(smallvec![TokenType::Func, TokenType::Eof]);
      if tok.is_err() {
        self.errs.push(tok.unwrap_err());
        self.sync(smallvec![TokenType::Func, TokenType::Eof]);
        continue;
      }
      let tok = tok.unwrap();

      match tok.token_type {
        TokenType::Func => {
          let res = self.parse_function();
          match res {
            Ok(f) => self.ast.borrow_mut().add_func(f),
            Err(errs) => {
              self.sync(smallvec![TokenType::Func, TokenType::Eof]);
              for e in errs.iter() {
                self.errs.push(e.clone())
              }
            }
          }
        }
        TokenType::Eof => break,
        _ => unreachable!(),
      }
    }

    if !self.errs.is_empty() {
      return Err(&self.errs);
    }

    Ok(self.ast.borrow())
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
