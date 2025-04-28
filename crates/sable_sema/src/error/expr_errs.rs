use ariadne::{Color, Label, Report, ReportKind};
use sable_parser::{parser::error::ParseErrReport, position::Position};

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

pub enum SemaExprError<'s> {
  VariableNotFound(VariableNotFound<'s>),
}

impl<'s> SemaExprError<'s> {
  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    match self {
      SemaExprError::VariableNotFound(err) => err.report(filename),
    }
  }
}
