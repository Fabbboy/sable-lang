use std::{
  cell::{Ref, RefCell},
  rc::Rc,
};

use sable_parser::ast::ast::AST;

use crate::mir::{builder::MirBuilder, error::MirError, module::MirModule};

pub struct Lowering<'s> {
  module: RefCell<MirModule<'s>>,
  ast: Rc<RefCell<AST<'s>>>,
  builder: MirBuilder<'s>,
  errs: Vec<MirError>,
}

impl<'s> Lowering<'s> {
  pub fn new(name: &'s str, ast: Rc<RefCell<AST<'s>>>) -> Self {
    Lowering {
      module: RefCell::new(MirModule::new(name)),
      ast,
      errs: vec![],
      builder: MirBuilder::new(),
    }
  }

  pub fn lower(&mut self) -> Result<Ref<MirModule<'s>>, &[MirError]> {
    if !self.errs.is_empty() {
      return Err(&self.errs);
    }

    Ok(self.module.borrow())
  }
}
