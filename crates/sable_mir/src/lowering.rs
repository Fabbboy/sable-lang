use std::{cell::RefCell, rc::Rc};

use sable_parser::ast::{ast::AST, function::Function};
use smallvec::SmallVec;

use crate::{
  error::LoweringError,
  mir::{
    function::{MirFunction, MirFunctionId, block::MirBlock},
    instruction::MirInstId,
    module::MirModule,
  },
};

const MAX_INLINE_FUNCS: usize = 20;

pub struct Lowerer<'ctx> {
  mir_mod: MirModule<'ctx>,
  ast: Rc<RefCell<AST<'ctx>>>,
  errors: Vec<LoweringError>,
}

impl<'ctx> Lowerer<'ctx> {
  pub fn new(mir_mod: MirModule<'ctx>, ast: Rc<RefCell<AST<'ctx>>>) -> Self {
    Self {
      mir_mod,
      ast,
      errors: Vec::new(),
    }
  }

  fn get_last_inst(&self, func: MirFunctionId) -> MirInstId {
    let func = match self.mir_mod.get_func(func) {
      Some(f) => f,
      None => return MirInstId(0),
    };

    let lst_blk_id = match func.get_last_blk() {
      Some(b) => b,
      None => return MirInstId(0),
    };

    let lst_blk = match func.get_block(lst_blk_id) {
      Some(b) => b,
      None => return MirInstId(0),
    };

    return MirInstId(lst_blk.range().end);
  }

  pub fn lower_func(&mut self, func: Rc<Function<'ctx>>) -> Result<(), Vec<LoweringError>> {
    let mut errors = Vec::new();

    let mir_func = MirFunction::new(func.get_name());
    let func_id = self.mir_mod.add_func(mir_func);

    let entry_block = MirBlock::new("entry", self.get_last_inst(func_id));
    let entry_block_id = self
      .mir_mod
      .get_func_mut(func_id)
      .unwrap()
      .add_block(entry_block);

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  pub fn lower(&mut self) -> Result<&MirModule, &[LoweringError]> {
    let funcs = {
      let ast = self.ast.borrow();
      let funcs = ast
        .get_funcs()
        .iter()
        .map(|f| f.clone())
        .collect::<SmallVec<[_; MAX_INLINE_FUNCS]>>();
      funcs
    };

    for func in funcs {
      let res = self.lower_func(func.clone());
      if let Err(errs) = res {
        self.errors.extend(errs);
      }
    }

    if self.errors.is_empty() {
      Ok(&self.mir_mod)
    } else {
      Err(&self.errors)
    }
  }
}
