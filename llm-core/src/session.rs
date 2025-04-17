use reqwest::RequestBuilder;

use crate::{
  message::{Message, Messages},
  model::DefaultModel,
  request::Request,
  response::Response,
};

#[derive(Debug, Clone)]
pub struct DefaultSessionOptions {
  pub(crate) model: DefaultModel,
}

impl DefaultSessionOptions {
  pub fn new(model_type: DefaultModel) -> Self {
    DefaultSessionOptions { model: model_type }
  }
}

#[derive(Debug)]
pub struct DefaultSession {
  model: DefaultModel,
  messages: Messages,
  request: RequestBuilder,
}

impl DefaultSession {
  pub(crate) fn new(options: DefaultSessionOptions, request: RequestBuilder) -> Self {
    DefaultSession {
      model: options.model,
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
    let payload = self.create_payload();
    let request = self.request.try_clone().unwrap();
    let res = request
      .json(&payload)
      .send()
      .await
      .unwrap()
      .json::<Response>()
      .await
      .unwrap();
    res
  }

  fn create_payload(&self) -> Request {
    let mut request = Request::new(self.model.real_name(), self.messages.message_ref());
    request.max_tokens = Some(10);
    request.stream = Some(false);
    request
  }
}
