use serde::Deserialize;

use crate::message::Message;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Response {
  Success(Answer),
  Failure(Failure),
}

impl Response {
  pub fn is_success(&self) -> bool {
    matches!(self, Response::Success(_))
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Answer {
  #[serde(flatten)]
  choices: Choices,
  model: String,
  id: String,
  created: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Failure {
  request_id: String,
  error: FailureError,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FailureError {
  code: String,
  message: String,
  param: Option<String>,
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
