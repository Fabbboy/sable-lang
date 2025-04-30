use std::{cell::RefCell, collections::HashMap, rc::Rc};

use sable_parser::{
  ast::{
    ast::AST,
    expression::{AssignExpression, Expression, LiteralExpression},
    function::Function,
    statement::{LetStatement, Statement},
  },
  info::ValType,
};
use smallvec::SmallVec;

use crate::{
  error::LoweringError,
  mir::{
    builder::Builder,
    function::{MirFunction, MirFunctionId, block::MirBlock},
    instruction::MirInstId,
    module::MirModule,
    value::{Constant, MirValue},
  },
};

const MAX_INLINE_FUNCS: usize = 20;

pub struct Lowerer<'ctx> {
  mir_mod: Rc<RefCell<MirModule<'ctx>>>,
  ast: Rc<RefCell<AST<'ctx>>>,
  errors: Vec<LoweringError<'ctx>>,
  namend: HashMap<&'ctx str, MirInstId>,
}

impl<'ctx> Lowerer<'ctx> {
  pub fn new(mir_mod: MirModule<'ctx>, ast: Rc<RefCell<AST<'ctx>>>) -> Self {
    Self {
      mir_mod: Rc::new(RefCell::new(mir_mod)),
      ast,
      errors: Vec::new(),
      namend: HashMap::new(),
    }
  }

  fn get_last_inst(&self, func: MirFunctionId) -> MirInstId {
    let module = self.mir_mod.borrow();

    let func = match module.get_func(func) {
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

  fn lower_literal_expression(
    &mut self,
    literal_expression: &LiteralExpression<'ctx>,
  ) -> Result<MirValue, LoweringError<'ctx>> {
    match literal_expression.get_type() {
      ValType::I32 => {
        let value = literal_expression.get_value().parse::<u64>();
        if value.is_err() {
          return Err(LoweringError::InvalidNumericValue(
            literal_expression.get_value(),
          ));
        }
        let value = value.unwrap();
        let value = MirValue::Constant(Constant::IntValue(literal_expression.get_type(), value));
        Ok(value)
      }
      ValType::F32 => {
        let value = literal_expression.get_value().parse::<f64>();
        if value.is_err() {
          return Err(LoweringError::InvalidNumericValue(
            literal_expression.get_value(),
          ));
        }
        let value = value.unwrap();
        let value = MirValue::Constant(Constant::FloatValue(literal_expression.get_type(), value));
        Ok(value)
      }
      ValType::Untyped => Err(LoweringError::IllegalType(literal_expression.get_type())),
      ValType::Void => Ok(MirValue::Constant(Constant::Null)),
    }
  }

  fn lower_assign_expression(
    &mut self,
    assign_expression: &AssignExpression<'ctx>,
    builder: &mut Builder<'ctx>,
  ) -> Result<MirValue, LoweringError<'ctx>> {
    match assign_expression.get_asignee() {
      Some(assign_to) => unimplemented!(),
      None => {
        let value = self.lower_expression(assign_expression.get_value(), builder)?;
        Ok(value)
      }
    }
  }

  fn lower_expression(
    &mut self,
    expr: &Expression<'ctx>,
    builder: &mut Builder<'ctx>,
  ) -> Result<MirValue, LoweringError<'ctx>> {
    match expr {
      Expression::LiteralExpression(literal_expression) => {
        self.lower_literal_expression(literal_expression)
      }
      Expression::BlockExpression(block_expression) => todo!(),
      Expression::AssignExpression(assign_expression) => {
        self.lower_assign_expression(assign_expression, builder)
      }
      Expression::VariableExpression(variable_expression) => todo!(),
      Expression::BinaryExpression(binary_expression) => todo!(),
      Expression::NullExpression(null_expression) => Ok(MirValue::Constant(Constant::Null)),
      Expression::CallExpression(call_expression) => todo!(),
    }
  }

  fn lower_let_stmt(
    &mut self,
    let_statement: &LetStatement<'ctx>,
    builder: &mut Builder<'ctx>,
  ) -> Result<(), LoweringError<'ctx>> {
    let store_loc = builder.build_alloca(let_statement.get_type().clone());
    let value = match let_statement.get_assignee() {
      Some(v) => self.lower_assign_expression(v, builder),
      None => Ok(MirValue::Constant(Constant::Null)),
    };

    if let Err(err) = value {
      return Err(err);
    }
    let value = value.unwrap();
    builder.build_store(store_loc, value);
    self.namend.insert(let_statement.get_name(), store_loc);

    Ok(())
  }

  fn lower_statement(
    &mut self,
    stmt: &Statement<'ctx>,
    builder: &mut Builder<'ctx>,
  ) -> Result<(), LoweringError<'ctx>> {
    match stmt {
      Statement::Expression(expression) => {
        let value = self.lower_expression(expression, builder);
        if let Err(err) = value {
          return Err(err);
        }
        Ok(())
      }
      Statement::ReturnStatement(return_statement) => todo!(),
      Statement::LetStatement(let_statement) => {
        let res = self.lower_let_stmt(let_statement, builder);
        if let Err(err) = res {
          return Err(err);
        }

        Ok(())
      }
    }
  }

  fn lower_func(&mut self, func: Rc<Function<'ctx>>) -> Result<(), Vec<LoweringError<'ctx>>> {
    let mut errors = Vec::new();

    let mir_func = MirFunction::new(func.get_name());
    let func_id = self.mir_mod.borrow_mut().add_func(mir_func);

    let entry_block = MirBlock::new("entry", self.get_last_inst(func_id));
    let entry_block_id = self
      .mir_mod
      .borrow_mut()
      .get_func_mut(func_id)
      .unwrap()
      .add_block(entry_block);

    let mut builder = Builder::new(self.mir_mod.clone(), func_id);
    builder.set_selected(entry_block_id);

    let stmts = func.get_body().get_stmts();

    for stmt in stmts {
      let res = self.lower_statement(&stmt, &mut builder);
      if let Err(errs) = res {
        errors.push(errs);
      }
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  pub fn lower(&mut self) -> Result<Rc<RefCell<MirModule<'ctx>>>, &[LoweringError]> {
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
      Ok(self.mir_mod.clone())
    } else {
      Err(&self.errors)
    }
  }
}
