use ariadne::Source;
use sable_parser::{lexer::lexer::Lexer, parser::parser::Parser};

const SOURCE: &str = r#"
func i32 main () { 
   // you can declare variables without initializing them
   var i32 counter;
   // or redeclare them
   // you can also just declare them with initializer
   var i32 counter = 123;
   return 2;
}
"#;
const FILENAME: &str = "test.sbl";

fn main() {
  let mut lexer = Lexer::new(SOURCE);
  let mut parser = Parser::new(&mut lexer);
  let parse_res = parser.parse();
  match parse_res {
    Ok(ast) => println!("Parsed successfully: {:#?}", ast),
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
