use ariadne::Source;
use sable_parser::{lexer::lexer::Lexer, parser::parser::Parser};

const SOURCE: &str = r#"
func i32 add(i32 x, i32 y) {
	return x + y;
}

//or without return (works only with expressions)
func i32 add(i32 x) {
  x + x;
}

// no parameters also works
func void useless() {
	return null;
}
"#;
const FILENAME: &str = "test.sbl";

fn main() {
  let mut lexer = Lexer::new(SOURCE);
  let mut parser = Parser::new(&mut lexer);
  let parse_res = parser.parse();
  match parse_res {
    Ok(ast) => {
      let serialized = serde_json::to_string_pretty(&*ast).unwrap();
      println!("AST: {serialized}");
    },
    Err(errs) => {
      for err in errs {
        err
          .report(FILENAME)
          .print((FILENAME, Source::from(SOURCE)))
          .unwrap();
      }
    }
  }
}
