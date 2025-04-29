use pretty::RcDoc;

#[derive(Debug)]
pub enum MirError<'s> {
  NoActiveBlock,
  BlockNotFound(usize),
  ValueMustBeTyped,
  InvalidNumericValue(&'s str),
  UndefinedVariable(&'s str),
}

impl<'s> MirError<'s> {
  pub fn report(&self) -> pretty::RcDoc {
    match self {
      MirError::NoActiveBlock => RcDoc::text("No active block"),
      MirError::BlockNotFound(block) => RcDoc::text(format!("Block not found: {}", block)),
      MirError::ValueMustBeTyped => RcDoc::text("Value must be typed"),
      MirError::InvalidNumericValue(value) => {
        RcDoc::text(format!("Invalid numeric value: {}", value))
      }
      MirError::UndefinedVariable(var) => RcDoc::text(format!("Undefined variable: {}", var)),
    }
  }
}
