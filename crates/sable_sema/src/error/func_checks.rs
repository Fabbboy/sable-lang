use ariadne::{Color, Label, Report, ReportKind};
use sable_parser::{parser::error::ParseErrReport, position::Position};

pub struct FunctionNotFound<'s> {
  name: &'s str,
  pos: Position,
}

impl<'s> FunctionNotFound<'s> {
  pub fn new(name: &'s str, pos: Position) -> Self {
    Self { name, pos }
  }

  pub fn name(&self) -> &'s str {
    self.name
  }

  pub fn pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    Report::build(ReportKind::Error, (filename, self.pos.range.clone()))
      .with_message(format!("function `{}` not found", self.name))
      .with_label(
        Label::new((filename, self.pos.range.clone()))
          .with_message("not found here")
          .with_color(Color::Yellow),
      )
      .finish()
  }
}

pub struct FunctionArgumentMismatch<'s> {
  name: &'s str,
  expected: usize,
  found: usize,
  pos: Position,
}

impl<'s> FunctionArgumentMismatch<'s> {
  pub fn new(name: &'s str, expected: usize, found: usize, pos: Position) -> Self {
    Self {
      name,
      expected,
      found,
      pos,
    }
  }

  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    Report::build(ReportKind::Error, (filename, self.pos.range.clone()))
      .with_message(format!(
        "function `{}` expected {} arguments, found {}",
        self.name, self.expected, self.found
      ))
      .with_label(
        Label::new((filename, self.pos.range.clone()))
          .with_message("argument mismatch here")
          .with_color(Color::Yellow),
      )
      .finish()
  }
}

pub enum FunctionCheckError<'s> {
  FunctionNotFound(FunctionNotFound<'s>),
  FunctionArgumentMismatch(FunctionArgumentMismatch<'s>),
}

impl<'s> FunctionCheckError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    match self {
      FunctionCheckError::FunctionNotFound(err) => err.report(filename),
      FunctionCheckError::FunctionArgumentMismatch(err) => err.report(filename),
    }
  }
}
