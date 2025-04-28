use ariadne::{Label, Report, ReportKind};
use sable_parser::{parser::error::ParseErrReport, position::Position};

pub struct VoidIllegal {
  pos: Position,
}

impl VoidIllegal {
  pub fn new(pos: Position) -> Self {
    Self { pos }
  }

  pub fn report<'f>(&self, filename: &'f str) -> ParseErrReport<'f> {
    Report::build(ReportKind::Error, (filename, self.pos.range.clone()))
      .with_message(format!("illegal use of void type"))
      .with_label(
        Label::new((filename, self.pos.range.clone()))
          .with_message("void type cannot be used here")
          .with_color(ariadne::Color::Yellow),
      )
      .finish()
  }
}
