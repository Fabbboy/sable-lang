use sable_parser::{
  ast::expression::{AssignExpression, BinaryExpression, Expression, VariableExpression},
  info::ValType,
};

use crate::sema::Sema;

pub fn infer_expr<'s>(analyzer: &mut Sema<'s>, expr: &Expression) -> ValType {
  match expr {
    Expression::LiteralExpression(literal_expression) => literal_expression.get_type(),
    Expression::BlockExpression(_) => ValType::Untyped,
    Expression::AssignExpression(assign_expression) => {
      infer_assign_expression(analyzer, assign_expression)
    }
    Expression::VariableExpression(variable_expression) => {
      infer_variable_expression(analyzer, variable_expression)
    }
    Expression::BinaryExpression(binary_expression) => {
      infer_binary_expression(analyzer, binary_expression)
    }
    Expression::NullExpression(_) => ValType::Void,
  }
}

pub fn infer_assign_expression<'s>(
  analyzer: &mut Sema<'s>,
  assign_expression: &AssignExpression,
) -> ValType {
  let val = assign_expression.get_value();
  let val_type = infer_expr(analyzer, val);
  val_type
}

pub fn infer_variable_expression<'s>(
  analyzer: &mut Sema<'s>,
  variable_expression: &VariableExpression,
) -> ValType {
  let name = variable_expression.get_name();
  if analyzer.resolver.is_declared(name) {
    let val_type = analyzer.resolver.resolve_var(name).unwrap();
    return val_type.get_type().clone();
  }
  ValType::Untyped
}

pub fn infer_binary_expression<'s>(
  analyzer: &mut Sema<'s>,
  binary_expression: &BinaryExpression,
) -> ValType {
  let lhs = infer_expr(analyzer, binary_expression.get_left());
  let rhs = infer_expr(analyzer, binary_expression.get_right());

  if lhs == rhs {
    return lhs;
  }

  ValType::Untyped
}
