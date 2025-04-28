use std::{cell::RefCell, rc::Rc};

use sable_parser::{
  ast::{ast::AST, expression::BlockExpression, function::Function, statement::Statement},
  info::ValType,
};

use crate::{
  builder::Builder,
  error::{LoweringError, VoidIllegal},
  mir::{
    function::{MirBlock, MirFunction, MirParam},
    module::MirModule,
  },
};

pub struct Lowerer<'s> {
  module: Rc<RefCell<MirModule<'s>>>,
  ast: Rc<RefCell<AST<'s>>>,
  errors: Vec<LoweringError>,
  builder: Builder<'s>,
}

impl<'s> Lowerer<'s> {
  pub fn new(name: &'s str, ast: Rc<RefCell<AST<'s>>>) -> Self {
    let module = Rc::new(RefCell::new(MirModule::new(name)));
    Self {
      module: module.clone(),
      ast,
      errors: Vec::new(),
      builder: Builder::new(module),
    }
  }

  fn type_sanity_check(type_: ValType) -> bool {
    match type_ {
      ValType::Void => false,
      _ => true,
    }
  }

  fn lower_stmt(&mut self, stmt: &Statement<'s>) -> Result<(), LoweringError> {
    Ok(())
  }

  fn lower_block(
    &mut self,
    f: usize,
    block: &BlockExpression<'s>,
  ) -> Result<(), Vec<LoweringError>> {
    let entry_block_idx = {
      let mut module = self.module.borrow_mut();
      let f = module.get_function_mut(f).unwrap();
      let block_idx = MirBlock::new(f, "entry");
      block_idx
    };
    self.builder.set_active_block(entry_block_idx);

    let mut errors = Vec::new();
    for (_, stmt) in block.get_stmts().iter().enumerate() {
      match self.lower_stmt(stmt) {
        Ok(_) => {}
        Err(err) => errors.push(err),
      }
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  fn lower_function(&mut self, function: Rc<Function<'s>>) -> Result<(), Vec<LoweringError>> {
    let errs = Vec::new();

    let name = function.get_name();
    let ret_type = function.get_ret_type();
    let mir_func_idx = MirFunction::new(self.module.borrow_mut(), name, ret_type.clone());
    for arg in function.get_params() {
      let mut module = self.module.borrow_mut();
      let mir_func = module.get_function_mut(mir_func_idx).unwrap();

      let name = arg.get_name();
      let type_ = arg.get_val_type();
      if !Self::type_sanity_check(type_.clone()) {
        let err = LoweringError::VoidIllegal(VoidIllegal::new(arg.get_pos()));
        self.errors.push(err);
        continue;
      }

      let param = MirParam::new(name, type_);
      mir_func.add_argument(param);
    }

    match self.lower_block(mir_func_idx, function.get_body()) {
      Ok(_) => {}
      Err(err) => {
        self.errors.extend(err);
      }
    }

    if errs.is_empty() { Ok(()) } else { Err(errs) }
  }

  pub fn lower(&mut self) -> Result<Rc<RefCell<MirModule<'s>>>, &Vec<LoweringError>> {
    let functions = {
      let ast_ref = self.ast.borrow();
      ast_ref.get_funcs().to_vec()
    };

    for function in functions {
      match self.lower_function(function) {
        Ok(_) => {}
        Err(errs) => {
          self.errors.extend(errs);
        }
      }
    }

    if self.errors.is_empty() {
      Ok(self.module.clone())
    } else {
      Err(&self.errors)
    }
  }
}
