pub mod void_illegal;

use sable_parser::parser::error::ParseErrReport;
pub use void_illegal::VoidIllegal;

pub enum LoweringError {
  VoidIllegal(void_illegal::VoidIllegal),
}

impl LoweringError {
  pub fn report<'f>(&self, filename: &'f str) -> ParseErrReport<'f> {
    match self {
      LoweringError::VoidIllegal(err) => err.report(filename),
    }
  }
}
