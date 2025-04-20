use reqwest::{Client, header::CONTENT_TYPE};

use crate::{
  session::{Session, SessionOptions},
  token::counter::ClientTokenCounter,
};

#[derive(Debug, Clone)]
pub struct DefaultClientOptions {
  pub(crate) bearer_token: String,
}

impl DefaultClientOptions {
  pub fn new(bearer_token: impl Into<String>) -> Self {
    DefaultClientOptions {
      bearer_token: bearer_token.into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct DefaultClient {
  http_client: Client,
  bearer_token: String,
  usage_counter: ClientTokenCounter,
}

impl DefaultClient {
  pub fn new(options: DefaultClientOptions) -> Self {
    let client = Client::new();

    let res = DefaultClient {
      http_client: client,
      bearer_token: options.bearer_token,
      usage_counter: ClientTokenCounter::default(),
    };
    res
  }

  pub fn new_session(&self, options: SessionOptions) -> Session {
    let endpoint = options.model.endpoint();
    let request = self
      .http_client
      .post(endpoint)
      .header(CONTENT_TYPE, "application/json")
      .bearer_auth(self.bearer_token.as_str());
    Session::new(options, request, self.usage_counter.clone())
  }
}
