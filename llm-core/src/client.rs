use reqwest::{Client, header::CONTENT_TYPE};

use crate::{
  errors::Result,
  session::{DefaultSession, DefaultSessionOptions},
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
}

impl DefaultClient {
  pub fn new(options: DefaultClientOptions) -> Self {
    let client = Client::new();

    let res = DefaultClient {
      http_client: client,
      bearer_token: options.bearer_token,
    };
    res
  }

  pub fn new_session(&self, options: DefaultSessionOptions) -> Result<DefaultSession> {
    let endpoint = options.model.endpoint();
    let request = self
      .http_client
      .post(endpoint)
      .header(CONTENT_TYPE, "application/json")
      .bearer_auth(self.bearer_token.as_str());
    let session = DefaultSession::new(options, request);
    Ok(session)
  }
}
