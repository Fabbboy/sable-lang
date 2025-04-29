use pretty::RcDoc;

#[derive(Debug)]
pub enum MirError {
  NoActiveBlock,
  BlockNotFound(usize),
}

impl MirError {
  pub fn report(&self) -> pretty::RcDoc {
    match self {
      MirError::NoActiveBlock => RcDoc::text("No active block"),
      MirError::BlockNotFound(block) => RcDoc::text(format!("Block not found: {}", block)),
    }
  }
}
