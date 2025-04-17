use reqwest::{header, Client};
use llm_core::model::Model;
use llm_core::session::Session;

pub struct AliyunClient {
    api_key: String,
    http_client: Client,
}

impl AliyunClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        let client = Client::new();
        AliyunClient {
            api_key: api_key.into(),
            http_client: client,
        }
    }

    pub fn session(&self, model: Model) -> Session {
        let mut req = self.http_client.post(model.endpoint());
        let req = req.bearer_auth(&self.api_key)
          .header(header::CONTENT_TYPE, "application/json");
        let session = Session::new(model, req);
        session
    }
}