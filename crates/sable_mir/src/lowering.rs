use std::{
  cell::{Ref, RefCell},
  rc::Rc,
};

use sable_parser::ast::{ast::AST, function::Function, statement::Statement};
use smallvec::{SmallVec, smallvec};

use crate::mir::{
  builder::MirBuilder,
  error::MirError,
  function::{MirBlock, MirFunction},
  module::MirModule,
};

const MAX_FUNCS: usize = 8;
const MAX_ERRORS: usize = 8;

pub struct Lowering<'s> {
  module: RefCell<MirModule<'s>>,
  ast: Rc<RefCell<AST<'s>>>,
  errs: SmallVec<[MirError; MAX_ERRORS]>,
}

impl<'s> Lowering<'s> {
  pub fn new(name: &'s str, ast: Rc<RefCell<AST<'s>>>) -> Self {
    Lowering {
      module: RefCell::new(MirModule::new(name)),
      ast,
      errs: smallvec![],
    }
  }

  fn lower_statement(
    &mut self,
    stmt: &Statement<'s>,
    func: Rc<Function<'s>>,
  ) -> Result<(), MirError> {
    Ok(())
  }

  fn lower_function(
    &mut self,
    func: Rc<Function<'s>>,
  ) -> Result<MirFunction<'s>, SmallVec<[MirError; MAX_ERRORS]>> {
    let mut errs = smallvec![];
    let mut mir_func = MirFunction::new(func.get_name());
    let block = func.get_body();

    let entry_block = MirBlock::new("entry");
    let entry_block_idx = mir_func.add_block(entry_block);

    let mut builder = MirBuilder::new(&mut mir_func);
    let res = builder.set_insert(entry_block_idx);
    if res.is_err() {
      return Err(smallvec![MirError::BlockNotFound(entry_block_idx)]);
    }

    for stmt in block.get_stmts() {
      let res = self.lower_statement(stmt, func.clone());
      match res {
        Ok(_) => {}
        Err(err) => {
          errs.push(err);
        }
      }
    }

    if errs.is_empty() {
      Ok(mir_func)
    } else {
      Err(errs)
    }
  }

  pub fn lower(&mut self) -> Result<Ref<MirModule<'s>>, &[MirError]> {
    let func_refs = self
      .ast
      .borrow()
      .get_funcs()
      .iter()
      .map(|f: &Rc<Function<'_>>| f.clone())
      .collect::<SmallVec<[_; MAX_FUNCS]>>();

    for func in func_refs {
      let res = self.lower_function(func.clone());
      match res {
        Ok(func) => {
          let mut module = self.module.borrow_mut();
          module.add_func(func);
        }
        Err(errs) => {
          self.errs.extend(errs);
        }
      }
    }

    if !self.errs.is_empty() {
      return Err(&self.errs);
    }

    Ok(self.module.borrow())
  }
}
