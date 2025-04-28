use ariadne::{Color, Label, Report, ReportKind};
use sable_parser::{info::ValType, parser::error::ParseErrReport, position::Position};

pub struct VariableNotFound<'s> {
  name: &'s str,
  pos: Position,
}

impl<'s> VariableNotFound<'s> {
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
      .with_message(format!("variable `{}` not found", self.name))
      .with_label(
        Label::new((filename, self.pos.range.clone()))
          .with_message("not found here")
          .with_color(Color::Yellow),
      )
      .finish()
  }
}

pub struct TypeMismatch {
  lhs: ValType,
  rhs: ValType,
  pos: Position,
}

impl TypeMismatch {
  pub fn new(lhs: ValType, rhs: ValType, pos: Position) -> Self {
    Self { lhs, rhs, pos }
  }

  pub fn report<'f>(&self, filename: &'f str) -> ParseErrReport<'f> {
    Report::build(ReportKind::Error, (filename, self.pos.range.clone()))
      .with_message(format!(
        "type mismatch: expected `{}`, found `{}`",
        self.lhs, self.rhs
      ))
      .with_label(
        Label::new((filename, self.pos.range.clone()))
          .with_message("mismatch here")
          .with_color(Color::Yellow),
      )
      .finish()
  }
}

pub struct IllegalNullVoid {
  pos: Position,
}

impl IllegalNullVoid {
  pub fn new(pos: Position) -> Self {
    Self { pos }
  }

  pub fn report<'f>(&self, filename: &'f str) -> ParseErrReport<'f> {
    Report::build(ReportKind::Error, (filename, self.pos.range.clone()))
      .with_message("illegal use of null or void")
      .with_label(
        Label::new((filename, self.pos.range.clone()))
          .with_message("null or void here")
          .with_color(Color::Yellow),
      )
      .finish()
  }
}

pub enum SemaExprError<'s> {
  VariableNotFound(VariableNotFound<'s>),
  TypeMismatch(TypeMismatch),
  IllegalNullVoid(IllegalNullVoid),
}

impl<'s> SemaExprError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    match self {
      SemaExprError::VariableNotFound(err) => err.report(filename),
      SemaExprError::TypeMismatch(err) => err.report(filename),
      SemaExprError::IllegalNullVoid(err) => err.report(filename),
    }
  }
}
