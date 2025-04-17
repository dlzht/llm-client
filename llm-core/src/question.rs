use serde::Serialize;

use crate::{
  common::{SearchOptions, StreamOptions},
  message::Message,
};

#[derive(Debug, Clone, Serialize)]
pub struct Question<'a> {
  pub model: &'a str,
  pub messages: &'a [Message],

  #[serde(rename = "stream")]
  pub enable_stream: Option<bool>,

  pub stream_options: Option<QuestionStreamOptions>,
  pub enable_search: Option<bool>,
  pub search_options: Option<QuestionSearchOptions>,
  pub temperature: Option<f32>,

  #[serde(rename = "max_tokens")]
  pub max_output_tokens: Option<i32>,

  #[serde(rename = "n")]
  pub output_res_count: Option<i32>,

  pub seed: Option<i32>,
  pub stop: Option<&'a [String]>,
}

impl<'a> Question<'a> {
  pub fn new(model: &'a str, messages: &'a [Message]) -> Self {
    Question {
      model,
      messages,
      enable_stream: None,
      stream_options: None,
      temperature: None,
      max_output_tokens: None,
      output_res_count: None,
      seed: None,
      stop: None,
      enable_search: None,
      search_options: None,
    }
  }

  pub fn stream_options(mut self, options: Option<&StreamOptions>) -> Self {
    if let Some(stream_options) = options {
      self.enable_stream = Some(stream_options.enable_stream);
      self.stream_options = options.map(|o| o.into());
    }
    self
  }

  pub fn temperature(mut self, temperature: Option<f32>) -> Self {
    self.temperature = temperature;
    self
  }

  pub fn max_output_tokens(mut self, max_output_tokens: Option<i32>) -> Self {
    self.max_output_tokens = max_output_tokens;
    self
  }

  pub fn output_res_count(mut self, output_res_count: Option<i32>) -> Self {
    self.output_res_count = output_res_count;
    self
  }

  pub fn seed(mut self, seed: Option<i32>) -> Self {
    self.seed = seed;
    self
  }

  pub fn stop(mut self, stop: Option<&'a [String]>) -> Self {
    self.stop = stop;
    self
  }

  pub fn search_options(mut self, options: Option<&SearchOptions>) -> Self {
    if let Some(search_options) = options {
      self.enable_search = Some(search_options.enable_search);
    }
    self
  }
}

#[derive(Debug, Clone, Serialize)]
struct QuestionStreamOptions {
  include_usage: bool,
}

impl From<&StreamOptions> for QuestionStreamOptions {
  fn from(value: &StreamOptions) -> Self {
    QuestionStreamOptions {
      include_usage: value.get_include_usage(),
    }
  }
}

#[derive(Debug, Clone, Serialize)]
struct QuestionSearchOptions {}
