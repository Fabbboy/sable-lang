use std::ops::Range;

use phf::phf_map;

use crate::{info::ValType, position::Position};

use super::token::{Token, TokenData, TokenType};

const KEYWORDS: phf::Map<&'static str, (TokenType, Option<TokenData>)> = phf_map! {
    "i32" => (TokenType::Type, Some(TokenData::Type(ValType::I32))),
    "f32" => (TokenType::Type, Some(TokenData::Type(ValType::F32))),
    "func" => (TokenType::Func, None),
    "return" => (TokenType::Return, None),
    "var" => (TokenType::Var, None),
};

pub struct Lexer<'s> {
  source: &'s str,

  // Position
  current: usize,
  start: usize,
  line: usize,
  column: usize,

  // Tokens
  curr_tok: Token<'s>,
  next_tok: Token<'s>,
}

impl<'s> Lexer<'s> {
  pub fn new(source: &'s str) -> Self {
    let mut l = Self {
      source,
      current: 0,
      start: 0,
      line: 1,
      column: 1,
      curr_tok: Token::default(),
      next_tok: Token::default(),
    };
    l.lex();
    l
  }

  fn get_token(&self, token_type: TokenType) -> Token<'s> {
    Token::new(token_type, self.get_lexeme(), self.get_pos(), None)
  }

  fn get_token_with_data(&self, token_type: TokenType, data: Option<TokenData>) -> Token<'s> {
    Token::new(token_type, self.get_lexeme(), self.get_pos(), data)
  }

  fn get_lexeme(&self) -> &'s str {
    if self.current > self.start {
      &self.source[self.start..self.current]
    } else {
      ""
    }
  }

  fn get_pos(&self) -> Position {
    let range = Range {
      start: self.start,
      end: self.current,
    };
    Position::new(self.line, self.column, range)
  }

  fn get_char(&self) -> Option<char> {
    if self.current < self.source.len() {
      self.source[self.current..].chars().next()
    } else {
      None
    }
  }

  fn advance(&mut self) -> Option<char> {
    if self.current >= self.source.len() {
      return None;
    }

    let c = self.get_char().unwrap(); // Always safe because we checked bounds
    let len = c.len_utf8();

    self.current += len;
    self.column += 1;
    Some(c)
  }

  fn lex_trivial(&mut self) {
    while let Some(c) = self.get_char() {
      if c.is_whitespace() {
        self.advance();
        if c == '\n' {
          self.line += 1;
          self.column = 1;
        } else {
          self.column += 1;
        }
      } else {
        break;
      }
    }
  }

  fn lex_identifier(&mut self) -> Token<'s> {
    while let Some(c) = self.get_char() {
      if c.is_alphanumeric() || c == '_' {
        self.advance();
      } else {
        break;
      }
    }

    let lexeme = self.get_lexeme();
    if let Some(token_type) = KEYWORDS.get(lexeme) {
      return self.get_token_with_data(token_type.0.clone(), token_type.1.clone());
    }

    self.get_token(TokenType::Identifier)
  }

  fn lex_number(&mut self) -> Token<'s> {
    while let Some(c) = self.get_char() {
      if c.is_digit(10) {
        self.advance();
      } else {
        break;
      }
    }

    if self.get_char() == Some('.') {
      self.advance();
      while let Some(c) = self.get_char() {
        if c.is_digit(10) {
          self.advance();
        } else {
          break;
        }
      }
      return self.get_token_with_data(TokenType::Float, Some(TokenData::Type(ValType::F32)));
    }
    self.get_token_with_data(TokenType::Integer, Some(TokenData::Type(ValType::I32)))
  }

  fn lex_comment(&mut self) {
    while let Some(c) = self.get_char() {
      if c != '\n' {
        self.advance();
      } else {
        break;
      }
    }
  }

  fn next(&mut self) -> Token<'s> {
    self.lex_trivial();
    self.start = self.current;
    let c = self.get_char();
    if c.is_none() {
      return self.get_token(TokenType::Eof);
    }
    let c = c.unwrap();
    self.advance();

    match c {
      '\0' => self.get_token(TokenType::Eof),
      'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),
      '0'..='9' => self.lex_number(),
      '(' => self.get_token(TokenType::Paren(true)),
      ')' => self.get_token(TokenType::Paren(false)),
      '{' => self.get_token(TokenType::Brace(true)),
      '}' => self.get_token(TokenType::Brace(false)),
      ':' => self.get_token(TokenType::Colon),
      ',' => self.get_token(TokenType::Comma),
      ';' => self.get_token(TokenType::Semicolon),
      '=' => self.get_token(TokenType::Assign),
      '/' => {
        if self.get_char() == Some('/') {
          self.advance();
          self.lex_comment();
          return self.next();
        }
        unimplemented!("Unexpected character: {}", c);
      }
      _ => return self.get_token(TokenType::Err),
    }
  }

  pub fn lex(&mut self) -> Token<'s> {
    self.curr_tok = self.next_tok.clone();
    self.next_tok = self.next();
    self.curr_tok.clone()
  }

  pub fn peek(&self) -> Token<'s> {
    self.next_tok.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_lexer() {
    let source = "";
    let mut lexer = Lexer::new(source);

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Eof);
    assert_eq!(token.lexeme, "");
    assert_eq!(token.pos.line, 1);
    assert_eq!(token.pos.column, 1);
    assert_eq!(token.pos.range, Range { start: 0, end: 0 });
  }

  #[test]
  fn test_lexing_literals() {
    let source = "abc 123 45.67";
    let mut lexer = Lexer::new(source);

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Identifier);
    assert_eq!(token.lexeme, "abc");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Integer);
    assert_eq!(token.lexeme, "123");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Float);
    assert_eq!(token.lexeme, "45.67");
  }

  #[test]
  fn test_lexing_types() {
    let source = "i32 f32";
    let mut lexer = Lexer::new(source);

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Type);
    assert_eq!(token.lexeme, "i32");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Type);
    assert_eq!(token.lexeme, "f32");
  }

  #[test]
  fn lex_symbols() {
    let source = "({:},)";
    let mut lexer = Lexer::new(source);

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Paren(true));
    assert_eq!(token.lexeme, "(");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Brace(true));
    assert_eq!(token.lexeme, "{");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Colon);
    assert_eq!(token.lexeme, ":");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Brace(false));
    assert_eq!(token.lexeme, "}");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Comma);
    assert_eq!(token.lexeme, ",");

    let token = lexer.lex();
    assert_eq!(token.token_type, TokenType::Paren(false));
    assert_eq!(token.lexeme, ")");
  }
}
