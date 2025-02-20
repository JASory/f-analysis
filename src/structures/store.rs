use crate::FResult;

pub trait Storage: Clone {

  fn to_persistent(&self, filename: &str) -> FResult<()>;
  
  fn from_persistent(x: &str) -> FResult<Self>;
}
