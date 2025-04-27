use std::cell::RefMut;

use sable_parser::ast::ast::AST;

use crate::error::AnalyzerError;

pub struct Sema<'a,'s> {
  ast: RefMut<'a, AST<'s>>,
  errors: Vec<AnalyzerError<'s>>,
}

impl<'a, 's> Sema<'a, 's> {
  pub fn new(ast: RefMut<'a, AST<'s>>) -> Self {
    Self {
      ast,
      errors: vec![],
    }
  }

  pub fn analyze(&mut self) -> Result<(), &Vec<AnalyzerError<'s>>> {
    if self.errors.is_empty() {
      Ok(())
    } else {
      Err(&self.errors)
    }
  }
}
