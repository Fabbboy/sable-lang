use std::rc::Rc;

use crate::{ast::ast::AST, lexer::lexer::Lexer};

use super::error::ParserError;

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
