use ariadne::Source;
use sable_parser::{lexer::lexer::Lexer, parser::parser::Parser};
use sable_sema::sema::Sema;

const SOURCE: &str = r#"
func i32 add(i32 x, i32 y) {
 let i32 x = 1;
  let i32 y = 2;
  return 2;
}
"#;
const FILENAME: &str = "test.sbl";

fn main() {
  let mut lexer = Lexer::new(SOURCE);
  let mut parser = Parser::new(&mut lexer);
  let parse_res = parser.parse();
  let ast = match parse_res {
    Ok(ast) => {
      let serialized = serde_json::to_string_pretty(&*ast).unwrap();
      println!("AST: {serialized}");
      ast
    }
    Err(errs) => {
      for err in errs {
        err
          .report(FILENAME)
          .print((FILENAME, Source::from(SOURCE)))
          .unwrap();
      }
      return;
    }
  };

  let mut sema = Sema::new(ast.clone());
  match sema.analyze() {
    Ok(_) => println!("No errors found."),
    Err(errors) => {
      for error in errors {
        error
          .report(FILENAME)
          .print((FILENAME, Source::from(SOURCE)))
          .unwrap();
      }
    }
  }
}
