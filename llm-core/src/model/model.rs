use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Model {
  inner: Arc<ModelInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ModelInner {
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
    let inner = ModelInner {
      real_name,
      nick_name,
      api_endpoint: api_endpoint.into(),
      description: None,
      // max_context_length: TokenLength::Unknown,
      // max_input_length: TokenLength::Unknown,
      // max_output_length: TokenLength::Unknown,
      // input_token_price: TokenPrice::Unknown,
      // output_token_price: TokenPrice::Unknown,
    };
    Model {
      inner: Arc::new(inner),
    }
  }

  pub fn real_name(&self) -> &str {
    &self.inner.real_name
  }

  pub fn endpoint(&self) -> &str {
    &self.inner.api_endpoint
  }

  pub fn inner(&self) -> &ModelInner {
    &self.inner
  }
}

impl From<ModelInner> for Model {
  fn from(value: ModelInner) -> Self {
    Model {
      inner: Arc::new(value),
    }
  }
}
