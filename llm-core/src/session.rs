use reqwest::RequestBuilder;

use crate::{
  answer::Response,
  common::{SearchOptions, StreamOptions},
  message::{Message, Messages},
  model::Model,
  question::Question,
};

#[derive(Debug, Clone)]
pub struct SessionOptions {
  pub(crate) model: Model,
  stream_options: Option<StreamOptions>,
  search_options: Option<SearchOptions>,
  temperature: Option<f32>,
  max_output_tokens: Option<i32>,
  output_res_count: Option<i32>,
  seed: Option<i32>,
  stop: Option<Vec<String>>,
}

impl SessionOptions {
  pub fn new(model: Model) -> Self {
    SessionOptions {
      model,
      stream_options: None,
      search_options: None,
      temperature: None,
      max_output_tokens: None,
      output_res_count: None,
      seed: None,
      stop: None,
    }
  }

  pub fn stream_options(mut self, stream_options: StreamOptions) -> Self {
    self.stream_options = Some(stream_options);
    self
  }

  pub fn search_options(mut self, search_options: SearchOptions) -> Self {
    self.search_options = Some(search_options);
    self
  }

  pub fn temperature(mut self, temperature: f32) -> Self {
    self.temperature = Some(temperature);
    self
  }

  pub fn max_output_tokens(mut self, max_output_tokens: i32) -> Self {
    self.max_output_tokens = Some(max_output_tokens);
    self
  }

  pub fn output_res_count(mut self, output_count: i32) -> Self {
    self.output_res_count = Some(output_count);
    self
  }

  pub fn seed(mut self, seed: i32) -> Self {
    self.seed = Some(seed);
    self
  }

  pub fn stop(mut self, stop: Vec<String>) -> Self {
    self.stop = Some(stop);
    self
  }
}

#[derive(Debug)]
pub struct Session {
  options: SessionOptions,
  messages: Messages,
  request: RequestBuilder,
}

impl Session {
  pub(crate) fn new(options: SessionOptions, request: RequestBuilder) -> Self {
    Session {
      options,
      messages: Messages::new(),
      request,
    }
  }

  pub(crate) fn system_message(&mut self, message: impl Into<String>) {
    let message = Message::system(message);
    self.messages.push(message);
  }

  pub(crate) fn user_message(&mut self, message: impl Into<String>) {
    let message = Message::user(message);
    self.messages.push(message);
  }

  pub fn play_as_assistant(&mut self, clear_history: bool) {
    if clear_history {
      self.messages.clear();
    }
    let message = Message::system("You are a helpful assistant.");
    self.messages.push(message);
  }

  pub fn play_as(&mut self, message: impl Into<String>, clear_history: bool) {
    if clear_history {
      self.messages.clear();
    }
    let message = Message::system(message);
    self.messages.push(message);
  }

  pub async fn ask_question(&mut self, question: impl Into<String>) -> Response {
    let message = Message::user(question);
    self.messages.push(message);
    let question = self.create_question();
    println!("{}", serde_json::to_string_pretty(&question).unwrap());
    let request = self.request.try_clone().unwrap();
    let res = request
      .json(&question)
      .send()
      .await
      .unwrap()
      // .text()
      .json::<Response>()
      .await
      .unwrap();
    res
  }

  fn create_question(&self) -> Question {
    Question::new(self.options.model.real_name(), self.messages.message_ref())
      .stream_options(self.options.stream_options.as_ref())
      .temperature(self.options.temperature)
      .max_output_tokens(self.options.max_output_tokens)
      .output_res_count(self.options.output_res_count)
      .seed(self.options.seed)
      .stop(self.options.stop.as_ref().map(|s| s.as_slice()))
      .search_options(self.options.search_options.as_ref())
  }
}
