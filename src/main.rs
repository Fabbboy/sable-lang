use ariadne::Source;
use sable_mir::mir::{
  function::{MirBlock, MirFunction},
  module::MirModule,
};
use sable_parser::{lexer::lexer::Lexer, parser::parser::Parser};
use sable_sema::sema::Sema;

const SOURCE: &str = r#"
func i32 add(i32 x, i32 y) {
  let i32 xy = x + add(x, y);
  return xy;
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

  let mut mir_mod = MirModule::new("test");
  {
    let add_idx = mir_mod.add_func(MirFunction::new("add"));
    let add_blk_idx = {
      let add_block = MirBlock::new("entry");
      let mut add_func = mir_mod.get_func_mut(add_idx).unwrap();
      add_func.add_block(add_block)
    };
    let mut add_builder = mir_mod.get_builder(add_idx).unwrap();
    add_builder.set_insert(add_blk_idx).unwrap();
  }

  println!("{:#?}", mir_mod);
}
