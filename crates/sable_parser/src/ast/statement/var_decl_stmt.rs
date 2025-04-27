use crate::{ast::expression::assign_expr::AssignExpression, info::ValType, position::Position};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VariableDeclStatement<'s> {
  type_: ValType,
  name: &'s str,
  assignee: Option<AssignExpression<'s>>,
  pos: Position,
}

impl<'s> VariableDeclStatement<'s> {
  pub fn new(
    type_: ValType,
    name: &'s str,
    assignee: Option<AssignExpression<'s>>,
    pos: Position,
  ) -> Self {
    Self {
      type_,
      name,
      assignee,
      pos,
    }
  }

  pub fn get_type(&self) -> &ValType {
    &self.type_
  }

  pub fn get_assignee(&self) -> &Option<AssignExpression<'s>> {
    &self.assignee
  }

  pub fn get_name(&self) -> &'s str {
    self.name
  }

  pub fn get_pos(&self) -> Position {
    let pos = self.pos.clone();
    if let Some(ref assignee) = self.assignee {
      pos.merge(assignee.get_pos())
    } else {
      pos
    }
  }
}
