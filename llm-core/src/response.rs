use serde::Deserialize;

use crate::message::Message;

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
  #[serde(flatten)]
  choices: Choices,
  model: String,
  id: String,
  created: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choices {
  choices: Vec<Choice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
  message: Message,
  finish_reason: String,
  index: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
  #[serde(rename(serialize = "prompt_tokens"))]
  input_token_length: i32,

  #[serde(rename(serialize = "completion_tokens"))]
  output_token_length: i32,
}
