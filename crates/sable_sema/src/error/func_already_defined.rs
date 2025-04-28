use ariadne::{Color, Label, Report, ReportKind};
use sable_parser::{parser::error::ParseErrReport, position::Position};

pub struct FunctionAlreadyDefined<'s> {
  name: &'s str,
  pos: Position,
  earlier: Position,
}

impl<'s> FunctionAlreadyDefined<'s> {
  pub fn new(name: &'s str, pos: Position, earlier: Position) -> Self {
    Self { name, pos, earlier }
  }

  pub fn name(&self) -> &'s str {
    self.name
  }

  pub fn pos(&self) -> Position {
    self.pos.clone()
  }

  pub fn report(&self, filename: &'s str) -> ParseErrReport<'s> {
    // The span of the *re*definition:
    let redefined = (filename, self.pos.range.clone());
    // The span of the *first* definition:
    let first_def = (filename, self.earlier.range.clone());

    Report::build(ReportKind::Error, redefined.clone())
      .with_message(format!("function `{}` is already defined", self.name))
      .with_label(
        Label::new(first_def)
          .with_message("first defined here")
          .with_color(Color::Yellow)
          .with_order(1),
      )
      .finish()
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use ariadne::Source;
  use sable_parser::{lexer::lexer::Lexer, parser::parser::Parser};

  const FILENAME: &str = "test.sbl";
  const SOURCE: &str = r#"
  func i32 my_func() {
    return 0;
  }
  "#;

  #[test]
  fn test_function_already_defined() {
    let mut lex = Lexer::new(SOURCE);
    let mut parse = Parser::new(&mut lex);
    let ast = parse.parse().unwrap();
    let ast_borrow = ast.borrow();
    let f = ast_borrow.get_funcs().get(0).unwrap();

    let err = FunctionAlreadyDefined::new(f.get_name(), f.get_pos().clone(), f.get_pos().clone());
    err
      .report(FILENAME)
      .print((FILENAME, Source::from(SOURCE)))
      .unwrap();
  }
}
