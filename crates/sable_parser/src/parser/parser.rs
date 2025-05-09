use std::{cell::RefCell, rc::Rc};

use smallvec::{SmallVec, smallvec};

use crate::{
  ast::{
    ast::AST,
    expression::{
      AssignExpression, BinaryExpression, BlockExpression, CallExpression, Expression,
      LiteralExpression, NullExpression, VariableExpression,
    },
    function::{Function, FunctionParameter},
    statement::{LetStatement, ReturnStatement, Statement},
  },
  info::ValType,
  lexer::{
    lexer::Lexer,
    token::{Token, TokenData, TokenType},
  },
  parser::error::lexer_err::LexerError,
  position::Position,
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
    let token = self.lexer.peek();
    if token.token_type == TokenType::Err {
      let err = LexerError::new(token);
      return Err(ParserError::LexerError(err));
    }

    for expected_token in expected.iter() {
      if token.token_type == *expected_token {
        return Ok(self.lexer.lex());
      }
    }

    let err = UnexpectedTokenError::new(expected, token);
    Err(ParserError::UnexpectedToken(err))
  }

  fn peek(&mut self, expected: SmallVec<[TokenType; MAX_EXPECTED]>) -> bool {
    let token = self.lexer.peek();
    if token.token_type == TokenType::Err {
      return false;
    }

    for expected_token in expected.iter() {
      if token.token_type == *expected_token {
        return true;
      }
    }

    false
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

  fn parse_assign(
    &mut self,
    name: Option<&'s str>,
  ) -> Result<AssignExpression<'s>, ParserError<'s>> {
    let tok = next!(@plain self, [TokenType::Assign]);
    let expr = self.parse_expression()?;
    let pos = tok.pos.merge(expr.get_pos());
    Ok(AssignExpression::new(name, expr, pos))
  }

  fn parse_call_expr(&mut self, name: Token<'s>) -> Result<Expression<'s>, ParserError<'s>> {
    next!(@plain self, [TokenType::Paren(true)]);
    let mut args = Vec::new();
    while !self.peek(smallvec![TokenType::Paren(false)]) {
      let arg = self.parse_expression()?;
      args.push(arg);
      if self.peek(smallvec![TokenType::Comma]) {
        next!(@plain self, [TokenType::Comma]);
      }
    }
    next!(@plain self, [TokenType::Paren(false)]);
    let pos = if args.is_empty() {
      name.pos
    } else {
      name.pos.merge(args.last().unwrap().get_pos())
    };
    let call_expr = Expression::CallExpression(CallExpression::new(pos, name.lexeme, args));
    Ok(call_expr)
  }

  fn parse_factor(&mut self) -> Result<Expression<'s>, ParserError<'s>> {
    let tok = next!(@plain self, [TokenType::Integer, TokenType::Float, TokenType::Identifier, TokenType::Null, TokenType::Paren(true)]);
    return match tok.token_type {
      TokenType::Integer | TokenType::Float => {
        let val = match tok.data {
          Some(TokenData::Type(ty)) => ty,
          _ => unreachable!(),
        };
        let lit = LiteralExpression::new(tok.lexeme, val, tok.pos);
        Ok(Expression::LiteralExpression(lit))
      }
      TokenType::Identifier => {
        let name = tok.lexeme;
        if self.peek(smallvec![TokenType::Assign]) {
          let expr = self.parse_assign(Some(name))?;
          return Ok(Expression::AssignExpression(expr));
        } else if self.peek(smallvec![TokenType::Paren(true)]) {
          let expr = self.parse_call_expr(tok)?;
          return Ok(expr);
        }
        let lit = VariableExpression::new(name, tok.pos);
        Ok(Expression::VariableExpression(lit))
      }
      TokenType::Null => {
        let lit = NullExpression::new(tok.pos);
        Ok(Expression::NullExpression(lit))
      }
      TokenType::Paren(true) => {
        let expr = self.parse_expression()?;
        next!(@plain self, [TokenType::Paren(false)]);
        Ok(expr)
      }
      _ => unreachable!(),
    };
  }

  fn parse_term(&mut self) -> Result<Expression<'s>, ParserError<'s>> {
    let lhs = self.parse_factor()?;

    if self.peek(smallvec![TokenType::Mul, TokenType::Div]) {
      let tok = next!(@plain self, [TokenType::Mul, TokenType::Div]);
      let operator = match tok.data {
        Some(TokenData::Operator(op)) => op,
        _ => unreachable!(),
      };
      let rhs = self.parse_term()?;
      let pos = tok.pos.merge(rhs.get_pos());
      let expr = Expression::BinaryExpression(BinaryExpression::new(lhs, operator, rhs, pos));
      return Ok(expr);
    }

    return Ok(lhs);
  }

  fn parse_expression(&mut self) -> Result<Expression<'s>, ParserError<'s>> {
    let lhs = self.parse_term()?;

    if self.peek(smallvec![TokenType::Plus, TokenType::Minus]) {
      let tok = next!(@plain self, [TokenType::Plus, TokenType::Minus]);
      let operator = match tok.data {
        Some(TokenData::Operator(op)) => op,
        _ => unreachable!(),
      };
      let rhs = self.parse_expression()?;
      let pos = tok.pos.merge(rhs.get_pos());
      let expr = Expression::BinaryExpression(BinaryExpression::new(lhs, operator, rhs, pos));
      return Ok(expr);
    }

    return Ok(lhs);
  }

  fn parse_variable_declaration(&mut self) -> Result<LetStatement<'s>, ParserError<'s>> {
    let type_ = next!(@plain self, [TokenType::Type]);
    let ty = match type_.data {
      Some(TokenData::Type(ty)) => ty,
      _ => unreachable!(),
    };

    let name = next!(@plain self, [TokenType::Identifier]);
    if self.peek(smallvec![TokenType::Semicolon]) {
      next!(@plain self, [TokenType::Semicolon]);
      let pos = type_.pos.merge(name.pos);
      let var_decl = LetStatement::new(ty, name.lexeme, None, pos);
      return Ok(var_decl);
    }

    if self.peek(smallvec![TokenType::Assign]) {
      let expr = self.parse_assign(None)?;
      let pos = type_.pos.merge(name.pos).merge(expr.get_pos());
      let var_decl = LetStatement::new(ty, name.lexeme, Some(expr), pos);
      next!(@plain self, [TokenType::Semicolon]);
      return Ok(var_decl);
    }

    let tok_res = self.next(smallvec![TokenType::Semicolon, TokenType::Comma]);
    if tok_res.is_ok() {
      unreachable!()
    }
    return Err(tok_res.unwrap_err());
  }

  fn parse_statement(&mut self) -> Result<Statement<'s>, ParserError<'s>> {
    if self.peek(smallvec![
      TokenType::Integer,
      TokenType::Float,
      TokenType::Identifier
    ]) {
      let expr = self.parse_expression()?;
      next!(@plain self, [TokenType::Semicolon]);
      let stmt = Statement::Expression(expr);
      return Ok(stmt);
    }

    let tok = next!(@plain self, [TokenType::Return, TokenType::Let]);
    return match tok.token_type {
      TokenType::Return => {
        let expr = self.parse_expression()?;
        let pos = tok.pos.merge(expr.get_pos());
        next!(@plain self, [TokenType::Semicolon]);
        let stmt = Statement::ReturnStatement(ReturnStatement::new(expr, ValType::Untyped, pos));
        Ok(stmt)
      }
      TokenType::Let => {
        let res = self.parse_variable_declaration();
        if res.is_err() {
          return Err(res.unwrap_err());
        }

        let var_decl = res.unwrap();
        Ok(Statement::LetStatement(var_decl))
      }
      _ => unreachable!(),
    };
  }

  fn parse_body(
    &mut self,
  ) -> Result<BlockExpression<'s>, SmallVec<[ParserError<'s>; MAX_EXPECTED]>> {
    let mut statements = Vec::new();
    let mut errors = SmallVec::new();

    let mut pos: Option<Position> = None;

    while !self.peek(smallvec![TokenType::Brace(false)]) {
      let statement = self.parse_statement();
      match statement {
        Ok(stmt) => {
          let stmt_pos = stmt.get_pos();
          match &mut pos {
            Some(p) => *p = p.merge(stmt_pos),
            None => pos = Some(stmt_pos),
          }
          statements.push(stmt);
        }
        Err(err) => {
          errors.push(err);
          self.sync(smallvec![TokenType::Semicolon, TokenType::Brace(false)]);
          if self.peek(smallvec![TokenType::Semicolon]) {
            next!(@vec self, [TokenType::Semicolon]);
          }
        }
      }
    }

    if errors.is_empty() {
      Ok(BlockExpression::new(statements, pos.unwrap_or_default()))
    } else {
      Err(errors)
    }
  }

  fn parse_param(&mut self) -> Result<FunctionParameter<'s>, ParserError<'s>> {
    let type_ = next!(@plain self, [TokenType::Type]);
    let ty = match type_.data {
      Some(TokenData::Type(ty)) => ty,
      _ => unreachable!(),
    };
    let name = next!(@plain self, [TokenType::Identifier]);
    let pos = type_.pos.merge(name.pos);
    Ok(FunctionParameter::new(name.lexeme, pos, ty))
  }

  fn parse_function(&mut self) -> Result<Function<'s>, SmallVec<[ParserError<'s>; MAX_EXPECTED]>> {
    let type_ = next!(@vec self, [TokenType::Type]);
    let ret_ty = match type_.data {
      Some(TokenData::Type(ty)) => ty,
      _ => unreachable!(),
    };
    let ty_pos = type_.pos;
    let name = next!(@vec self, [TokenType::Identifier]);
    next!(@vec self, [TokenType::Paren(true)]);
    let mut params = Vec::new();
    while !self.peek(smallvec![TokenType::Paren(false)]) {
      let param = self.parse_param();
      if param.is_err() {
        return Err(smallvec![param.unwrap_err()]);
      }
      let param = param.unwrap();
      params.push(param);
      if self.peek(smallvec![TokenType::Comma]) {
        next!(@vec self, [TokenType::Comma]);
      }
    }
    next!(@vec self, [TokenType::Paren(false)]);
    next!(@vec self, [TokenType::Brace(true)]);
    let body = self.parse_body();
    if body.is_err() {
      return Err(body.unwrap_err());
    }
    let body = body.unwrap();

    next!(@vec self, [TokenType::Brace(false)]);

    let ty_pos = ty_pos.merge(name.pos);
    Ok(Function::new(name.lexeme, params, ty_pos, ret_ty, body))
  }

  pub fn get_ast(&self) -> Rc<RefCell<AST<'s>>> {
    self.ast.clone()
  }

  pub fn parse(&mut self) -> Result<Rc<RefCell<AST<'s>>>, &[ParserError<'s>]> {
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
