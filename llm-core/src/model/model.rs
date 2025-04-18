use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub type ModelRef = Arc<Model>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
  real_name: String,
  nick_name: String,
  api_endpoint: String,
  description: Option<String>,
  // max_context_length: TokenLength,
  // max_input_length: TokenLength,
  // max_output_length: TokenLength,
  // input_token_price: TokenPrice,
  // output_token_price: TokenPrice,
}

impl Model {
  pub fn new(real_name: impl Into<String>, api_endpoint: impl Into<String>) -> Self {
    let real_name = real_name.into();
    let nick_name = real_name.clone();
    Model {
      real_name,
      nick_name,
      api_endpoint: api_endpoint.into(),
      description: None,
      // max_context_length: TokenLength::Unknown,
      // max_input_length: TokenLength::Unknown,
      // max_output_length: TokenLength::Unknown,
      // input_token_price: TokenPrice::Unknown,
      // output_token_price: TokenPrice::Unknown,
    }
  }

  pub fn real_name(&self) -> &str {
    self.real_name.as_str()
  }

  pub fn endpoint(&self) -> &str {
    &self.api_endpoint.as_str()
  }

}