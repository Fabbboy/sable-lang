use std::cell::RefMut;

use super::{error::MirError, function::MirFunction};

pub struct MirBuilder<'b, 's> {
  func: RefMut<'b, MirFunction<'s>>,
  ab: Option<usize>,
}

impl<'b, 's> MirBuilder<'b, 's> {
  pub fn new(func: RefMut<'b, MirFunction<'s>>) -> Self {
    MirBuilder { func, ab: None }
  }

  pub fn set_insert(&mut self, ab: usize) -> Result<(), MirError> {
    let blk = self.func.get_block(ab);
    if blk.is_none() {
      return Err(MirError::BlockNotFound(ab));
    }
    self.ab = Some(ab);
    Ok(())
  }
}
