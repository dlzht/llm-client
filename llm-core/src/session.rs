use futures_util::StreamExt;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Event, EventSource};
use snafu::{OptionExt, ResultExt};

use crate::{
  answer::{FluxRes, MonoRes},
  common::{SearchOptions, StreamOptions},
  errors::{
    DeserializeJsonSnafu, EventsourceSnafu, ImpossibleSnafu, PlainMessageSnafu, ReqwestClientSnafu,
    Result,
  },
  message::{Message, Messages, Role},
  model::ModelRef,
  question::Question,
  token::counter::{ClientTokenCounter, SessionTokenCounter},
};

pub enum Response {
  Single(MonoRes),
  Stream(StreamRes),
}

pub struct StreamRes {
  eventsource: EventSource,
}

impl StreamRes {
  pub fn new(eventsource: EventSource) -> Self {
    Self { eventsource }
  }

  pub async fn next(&mut self) -> Option<Result<FluxRes>> {
    loop {
      match self.inner_next().await {
        Some(Ok(Event::Open)) => continue,
        Some(Ok(Event::Message(event))) => {
          let data = event.data;
          if "[DONE]" == data.as_str() {
            self.eventsource.close();
            return None;
          }
          let res = serde_json::from_str::<FluxRes>(&data).context(DeserializeJsonSnafu);
          if res.is_err() {
            self.eventsource.close();
          }
          return Some(res);
        }
        Some(Err(_)) | None => {
          self.eventsource.close();
          return None;
        }
      }
    }
  }

  pub async fn inner_next(&mut self) -> Option<Result<Event>> {
    self
      .eventsource
      .next()
      .await
      .map(|res| res.context(EventsourceSnafu))
  }
}

#[derive(Debug, Clone)]
pub struct SessionOptions {
  pub(crate) model: ModelRef,
  stream_options: Option<StreamOptions>,
  search_options: Option<SearchOptions>,
  temperature: Option<f32>,
  max_output_tokens: Option<i32>,
  output_res_count: Option<i32>,
  seed: Option<i32>,
  stop: Option<Vec<String>>,
}

impl SessionOptions {
  pub fn new(model: ModelRef) -> Self {
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
  session_usage_counter: SessionTokenCounter,
  client_usage_counter: ClientTokenCounter,
}

impl Session {
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

  pub async fn ask_question(&mut self, question: impl Into<String>) -> Result<Response> {
    let message = Message::user(question);
    self.messages.push(message);
    let question = self.create_question();
    let request = self
      .request
      .try_clone()
      .context(ImpossibleSnafu)?
      .json(&question);

    if self.is_stream_mode() {
      return self.process_stream_question(request);
    }
    self.process_normal_question(request).await
  }
}

impl Session {
  pub(crate) fn new(
    options: SessionOptions,
    request: RequestBuilder,
    client_usage_counter: ClientTokenCounter,
  ) -> Self {
    Session {
      options,
      messages: Messages::new(),
      request,
      session_usage_counter: SessionTokenCounter::default(),
      client_usage_counter,
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

  fn is_stream_mode(&self) -> bool {
    self
      .options
      .stream_options
      .as_ref()
      .map(|o| o.enable_stream)
      .unwrap_or(false)
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

  async fn process_normal_question(&mut self, request: RequestBuilder) -> Result<Response> {
    let res = request
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<MonoRes>()
      .await
      .context(ReqwestClientSnafu)?;
    if let (Some((input_token, output_token)), Some(output_bytes)) =
      (res.token_usage(), res.output_bytes())
    {
      let input_bytes = self
        .last_user_message()
        .context(ImpossibleSnafu)?
        .content()
        .len();
      self.update_token_usage(input_bytes, input_token, output_bytes, output_token);
    }
    Ok(Response::Single(res))
  }

  fn process_stream_question(&self, request: RequestBuilder) -> Result<Response> {
    let eventsource = EventSource::new(request).map_err(|_| {
      PlainMessageSnafu {
        message: "Eventsource can not clone request".to_string(),
      }
      .build()
    })?;
    Ok(Response::Stream(StreamRes::new(eventsource)))
  }

  fn update_token_usage(
    &mut self,
    input_bytes: usize,
    input_token: usize,
    output_bytes: usize,
    output_token: usize,
  ) {
    self
      .session_usage_counter
      .incr_input(input_bytes, input_token);
    self
      .session_usage_counter
      .incr_output(output_bytes, output_token);
    self
      .client_usage_counter
      .incr_input(input_token, input_token);
    self
      .client_usage_counter
      .incr_output(output_token, output_token);
  }

  fn last_user_message(&self) -> Option<&Message> {
    self
      .messages
      .message_ref()
      .iter()
      .rev()
      .filter(|m| m.role() == Role::User)
      .next()
  }
}
