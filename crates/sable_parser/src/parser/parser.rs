use crate::{ast::ast::AST, lexer::lexer::Lexer};

pub struct Parser<'p, 's> {
  lexer: &'p mut Lexer<'s>,
  ast: AST,
}

impl<'p, 's> Parser<'p, 's> {
  pub fn new(lexer: &'p mut Lexer<'s>) -> Self {
    Parser {
      lexer,
      ast: AST::default(),
    }
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
}
