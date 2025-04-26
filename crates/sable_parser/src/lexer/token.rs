use crate::{info::ValType, position::Position};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenType {
  // Special
  #[default]
  Eof,
  Err,

  // Literal
  Identifier,
  Integer,
  Float,

  // Keywords
  Type,
  Func,
  Return,

  // Symbols
  Paren(bool),
  Brace(bool),
  Colon,
  Comma,
  Semicolon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
  Type(ValType),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Token<'s> {
  pub token_type: TokenType,
  pub lexeme: &'s str,
  pub pos: Position,
  pub data: Option<TokenData>,
}

impl<'s> Token<'s> {
  pub fn new(
    token_type: TokenType,
    lexeme: &'s str,
    pos: Position,
    data: Option<TokenData>,
  ) -> Self {
    Self {
      token_type,
      lexeme,
      pos,
      data,
    }
  }
}
