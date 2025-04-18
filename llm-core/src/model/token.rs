use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum TokenLength {
  Length(usize),
  Unknown,
}

impl TokenLength {
  pub fn new_zero() -> Self {
    TokenLength::Length(0)
  }

  pub fn new_unknown() -> Self {
    TokenLength::Unknown
  }

  pub fn is_unknown(&self) -> bool {
    matches!(self, TokenLength::Unknown)
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TokenPrice {
  Lowest(f32),
  Unknown,
}

impl TokenPrice {
  pub fn new_lowest(value: f32) -> Self {
    TokenPrice::Lowest(value)
  }

  pub fn new_unknown() -> Self {
    TokenPrice::Unknown
  }

  pub fn is_unknown(&self) -> bool {
    matches!(self, TokenPrice::Unknown)
  }
}
