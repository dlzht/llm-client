use std::fmt::{Debug, Display, Formatter};

pub enum Error {
  SerdeJson(serde_json::Error),
  Unknown,
}

impl Debug for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::SerdeJson(err) => Display::fmt(err, f),
      Error::Unknown => f.write_str("Unknown"),
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}

pub type Result<T> = std::result::Result<T, Error>;
