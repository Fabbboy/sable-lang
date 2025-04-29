use std::{
  cell::{RefCell, RefMut},
  rc::Rc,
};

use super::function::MirBlock;

pub struct MirBuilder<'s> {
  ab: Option<RefMut<'s, MirBlock<'s>>>,
}

impl<'s> MirBuilder<'s> {
  pub fn new() -> Self {
    MirBuilder { ab: None }
  }

  pub fn set_insert(&mut self, ab: RefMut<'s, MirBlock<'s>>) {
    self.ab = Some(ab);
  }
}
