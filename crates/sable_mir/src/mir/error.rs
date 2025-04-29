#[derive(Debug)]
pub enum MirError {
  FunctionNotFound(usize),
  BlockNotFound(usize),
}
