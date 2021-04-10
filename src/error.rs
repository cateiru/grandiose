use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("dont create error: {0}")]
  DontCreateError(String),
}
