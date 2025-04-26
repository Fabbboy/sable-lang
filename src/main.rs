use ariadne::Source;
use sable_parser::{lexer::lexer::Lexer, parser::parser::Parser};

const SOURCE: &str = r#"
func i32 main () { // here we can make a function invalid {
// this is a return statement
    return 2; // we might leave important information in the comments
    // i mean this comment will probably never be executed... wait what?
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
