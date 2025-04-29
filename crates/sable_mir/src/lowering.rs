use std::{
  cell::{Ref, RefCell},
  collections::HashMap,
  rc::Rc,
};

use sable_parser::{
  ast::{
    ast::AST,
    expression::{AssignExpression, Expression, LiteralExpression},
    function::Function,
    statement::{LetStatement, Statement},
  },
  info::ValType,
};
use smallvec::{SmallVec, smallvec};

use crate::mir::{
  builder::{self, MirBuilder},
  error::MirError,
  function::{MirBlock, MirFunction},
  module::MirModule,
  value::{Value, constant::Constant},
};

const MAX_FUNCS: usize = 8;
const MAX_ERRORS: usize = 8;

pub struct Lowering<'s> {
  module: RefCell<MirModule<'s>>,
  ast: Rc<RefCell<AST<'s>>>,
  errs: SmallVec<[MirError<'s>; MAX_ERRORS]>,
}

impl<'s> Lowering<'s> {
  pub fn new(name: &'s str, ast: Rc<RefCell<AST<'s>>>) -> Self {
    Lowering {
      module: RefCell::new(MirModule::new(name)),
      ast,
      errs: smallvec![],
    }
  }

  fn lower_literal_expression(
    &mut self,
    literal_expression: &LiteralExpression<'s>,
  ) -> Result<Value, MirError<'s>> {
    match literal_expression.get_type() {
      ValType::Untyped => Err(MirError::ValueMustBeTyped),
      ValType::I32 => {
        let value = literal_expression.get_value().parse::<i64>();
        match value {
          Ok(value) => Ok(Value::Constant(Constant::Int(value))),
          Err(_) => Err(MirError::InvalidNumericValue(
            literal_expression.get_value(),
          )),
        }
      }
      ValType::F32 => {
        let value = literal_expression.get_value().parse::<f64>();
        match value {
          Ok(value) => Ok(Value::Constant(Constant::Float(value))),
          Err(_) => Err(MirError::InvalidNumericValue(
            literal_expression.get_value(),
          )),
        }
      }
      ValType::Void => Ok(Value::Constant(Constant::Null)),
    }
  }

  fn lower_null_expression(&mut self) -> Value {
    Value::Constant(Constant::Null)
  }

  fn lower_assign_expression(
    &mut self,
    assign_expression: &AssignExpression<'s>,
    builder: &mut MirBuilder<'s, '_>,
  ) -> Result<Value, MirError<'s>> {
    match assign_expression.get_asignee() {
      Some(name) => unreachable!(),
      None => self.lower_expression(assign_expression.get_value(), builder),
    }
  }

  fn lower_expression(
    &mut self,
    expression: &Expression<'s>,
    builder: &mut MirBuilder<'s, '_>,
  ) -> Result<Value, MirError<'s>> {
    match expression {
      Expression::LiteralExpression(literal_expression) => {
        self.lower_literal_expression(literal_expression)
      }
      Expression::BlockExpression(block_expression) => todo!(),
      Expression::AssignExpression(assign_expression) => {
        self.lower_assign_expression(assign_expression, builder)
      }
      Expression::VariableExpression(variable_expression) => todo!(),
      Expression::BinaryExpression(binary_expression) => todo!(),
      Expression::NullExpression(_) => Ok(self.lower_null_expression()),
      Expression::CallExpression(call_expression) => todo!(),
    }
  }

  fn lower_let_statement(
    &mut self,
    let_statement: &LetStatement<'s>,
    builder: &mut MirBuilder<'s, '_>,
  ) -> Result<Value, MirError<'s>> {
    let value = match let_statement.get_assignee() {
      Some(assign_expression) => self.lower_assign_expression(assign_expression, builder),
      None => Ok(Value::Constant(Constant::Null)),
    };

    if value.is_err() {
      return Err(value.unwrap_err());
    }
    let value = value.unwrap();
    let inst = builder.create_define(
      let_statement.get_name(),
      let_statement.get_type().clone(),
      value,
    )?;

    Ok(inst)
  }

  fn lower_statement(
    &mut self,
    stmt: &Statement<'s>,
    builder: &mut MirBuilder<'s, '_>,
  ) -> Result<(), MirError<'s>> {
    match stmt {
      Statement::Expression(expression) => {
        let res = self.lower_expression(expression, builder);
        match res {
          Ok(_) => Ok(()),
          Err(err) => Err(err),
        }
      }
      Statement::ReturnStatement(return_statement) => todo!(),
      Statement::LetStatement(let_statement) => {
        let res = self.lower_let_statement(let_statement, builder);
        match res {
          Ok(_) => Ok(()),
          Err(err) => Err(err),
        }
      }
    }
  }

  fn lower_function(
    &mut self,
    func: Rc<Function<'s>>,
  ) -> Result<MirFunction<'s>, SmallVec<[MirError<'s>; MAX_ERRORS]>> {
    let mut errs = smallvec![];

    let ast_block = func.get_body();
    let mut mir_func = MirFunction::new(func.get_name());

    let entry_block = MirBlock::new("entry");
    let entry_block_idx = mir_func.add_block(entry_block);

    let mut builder = MirBuilder::new(&mut mir_func);

    let res = builder.set_insert(entry_block_idx);
    if res.is_err() {
      return Err(smallvec![MirError::BlockNotFound(entry_block_idx)]);
    }

    for stmt in ast_block.get_stmts() {
      let stmt = stmt;
      let res = self.lower_statement(stmt, &mut builder);
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

  pub fn lower(&mut self) -> Result<Ref<MirModule<'s>>, &[MirError<'s>]> {
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
