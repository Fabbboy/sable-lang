use sable_parser::{lexer::lexer::Lexer, parser::parser::Parser};

const SOURCE: &str = r#"

"#;

fn main() {
  let mut lexer = Lexer::new(SOURCE);
  let parser = Parser::new(&mut lexer);
  _ = parser;
}
