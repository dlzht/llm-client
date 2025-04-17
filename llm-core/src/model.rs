use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DefaultModel {
    inner: Arc<ModelInner>,
}

#[derive(Debug, Clone)]
pub struct ModelInner {
    real_name: String,
    nick_name: String,
    api_endpoint: String,
    max_context_length: TokenLength,
    max_input_length: TokenLength,
    max_output_length: TokenLength,
    input_token_price: TokenPrice,
    output_token_price: TokenPrice,
    description: Option<String>,
}

impl DefaultModel {
    pub fn new(real_name: impl Into<String>, api_endpoint: impl Into<String>) -> Self {
        let real_name = real_name.into();
        let nick_name = real_name.clone();
        let inner = ModelInner {
            real_name,
            nick_name,
            api_endpoint: api_endpoint.into(),
            max_context_length: TokenLength::Unknown,
            max_input_length: TokenLength::Unknown,
            max_output_length: TokenLength::Unknown,
            input_token_price: TokenPrice::Unknown,
            output_token_price: TokenPrice::Unknown,
            description: None,
        };
        DefaultModel { inner: Arc::new(inner) }
    }

    pub fn real_name(&self) -> &str {
        &self.inner.real_name
    }

    pub fn endpoint(&self) -> &str {
        &self.inner.api_endpoint
    }

}

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum TokenLength {
    Length(usize),
    Unknown
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


