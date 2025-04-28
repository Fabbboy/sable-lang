use ariadne::{Color, Label, Report, ReportKind};
use sable_parser::{parser::error::ParseErrReport, position::Position};

pub struct VariableRedeclared<'s> {
  name: &'s str,
  pos: Position,
  earlier_pos: Position,
}

impl<'s> VariableRedeclared<'s> {
  pub fn new(name: &'s str, pos: Position, earlier_pos: Position) -> Self {
    Self {
      name,
      pos,
      earlier_pos,
    }
  }

  pub fn name(&self) -> &'s str {
    self.name
  }

  pub fn pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    Report::build(ReportKind::Error, (filename, self.pos.range.clone()))
      .with_message(format!("variable `{}` is already declared", self.name))
      .with_label(
        Label::new((filename, self.earlier_pos.range.clone()))
          .with_message("first declared here")
          .with_color(Color::Yellow)
          .with_order(1),
      )
      .finish()
  }
}
