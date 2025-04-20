use serde::Deserialize;

use crate::message::Message;

#[derive(Debug, Clone, Deserialize)]
pub struct FluxRes {
  #[serde(flatten)]
  choices: FluxChoices,
  object: String,
  created: i32,
  model: String,
  id: String,
  usage: Option<Usage>,
}

impl FluxRes {
  pub fn token_usage(&self) -> Option<(usize, usize)> {
    self.usage.as_ref().map(|u| u.token_usage())
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FluxChoices {
  choices: Vec<FluxChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FluxChoice {
  delta: FluxDelta,
  index: i32,
  finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FluxDelta {
  content: String,
  role: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
  #[serde(rename = "prompt_tokens")]
  input_tokens: u32,

  #[serde(rename = "completion_tokens")]
  output_tokens: u32,

  #[serde(rename = "total_tokens")]
  total_token: u32,
}

impl Usage {
  pub fn token_usage(&self) -> (usize, usize) {
    (self.input_tokens as usize, self.output_tokens as usize)
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MonoRes {
  Success(Answer),
  Failure(Failure),
}

impl MonoRes {
  pub fn token_usage(&self) -> Option<(usize, usize)> {
    match self {
      MonoRes::Success(answer) => Some(answer.token_usage()),
      MonoRes::Failure(_) => None,
    }
  }

  pub fn output_bytes(&self) -> Option<usize> {
    match self {
      MonoRes::Success(answer) => Some(answer.output_bytes()),
      MonoRes::Failure(_) => None,
    }
  }
}

impl MonoRes {
  pub fn is_success(&self) -> bool {
    matches!(self, MonoRes::Success(_))
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Answer {
  #[serde(flatten)]
  choices: MonoChoices,
  model: String,
  id: String,
  created: u32,
  object: String,
  usage: Usage,
}

impl Answer {
  pub fn token_usage(&self) -> (usize, usize) {
    self.usage.token_usage()
  }

  pub fn output_bytes(&self) -> usize {
    self
      .choices
      .choices
      .iter()
      .map(|c| c.message.content().len())
      .sum()
  }
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
pub struct MonoChoices {
  choices: Vec<MonoChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MonoChoice {
  message: Message,
  finish_reason: String,
  index: i32,
}

#[cfg(test)]
mod test {
  use crate::answer::FluxRes;

  #[test]
  fn test_flux_res() {
    let json = "{\"choices\":[{\"finish_reason\":null,\"delta\":{\"content\":\"苹果\"},\"index\":0,\"logprobs\":null}],\"object\":\"chat.completion.chunk\",\"usage\":null,\"created\":1744906848,\"system_fingerprint\":null,\"model\":\"qwen-turbo\",\"id\":\"chatcmpl-187e0870-2a2f-932f-b5eb-12828ba98987\"}";
    let res = serde_json::from_str::<FluxRes>(json);
    assert!(matches!(res, Ok(FluxRes { .. })));

    let json = "{\"choices\":[],\"object\":\"chat.completion.chunk\",\"usage\":{\"prompt_tokens\":25,\"completion_tokens\":10,\"total_tokens\":35},\"created\":1744906848,\"system_fingerprint\":null,\"model\":\"qwen-turbo\",\"id\":\"chatcmpl-187e0870-2a2f-932f-b5eb-12828ba98987\"}";
    let res = serde_json::from_str::<FluxRes>(json);
    assert!(matches!(res, Ok(FluxRes { .. })));
  }
}
