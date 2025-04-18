use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::{Client as ReqwestClient, Client, RequestBuilder};
use serde::Deserialize;

use crate::{
  errors::Result,
  model::{Model, ModelInner, registry::registry::Registry},
};

#[async_trait]
pub trait RemoteClient {
  async fn search(&self, name: &str) -> Result<Vec<Model>>;

  async fn register(&mut self, model: Model) -> Result<()>;

  async fn deregister(&mut self, model: Model) -> Result<()>;
}

pub struct HttpClient {
  search: RequestBuilder,
  register: RequestBuilder,
  deregister: RequestBuilder,
}

const HTTP_PREFIX: &'static str = "https://";
const HTTPS_PREFIX: &'static str = "https://";

impl HttpClient {
  pub fn new_with_default(base_url: &str, client: ReqwestClient) -> Self {
    fn normalize_base_url(base_url: &str) -> String {
      let mut result = String::with_capacity(base_url.len());
      if !base_url.starts_with(HTTP_PREFIX) && !base_url.starts_with(HTTPS_PREFIX) {
        result.push_str(HTTPS_PREFIX);
      }
      result.push_str(base_url);
      if !base_url.ends_with("/") {
        result.push_str("/");
      }
      result
    }

    let base_url = normalize_base_url(base_url);
    HttpClient {
      search: client.get(format!("{base_url}search")),
      register: client.post(format!("{base_url}register")),
      deregister: client.post(format!("{base_url}deregister")),
    }
  }

  pub fn new_with_request(
    search: RequestBuilder,
    register: RequestBuilder,
    deregister: RequestBuilder,
  ) -> Self {
    HttpClient {
      search,
      register,
      deregister,
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct HttpRes<T: Deserialize + Debug> {
  code: i32,
  message: Option<String>,
  data: T,
}

#[async_trait]
impl RemoteClient for HttpClient {
  async fn search(&self, name: &str) -> Result<Vec<Model>> {
    let req = self.search.try_clone().unwrap();
    let req = req
      .query(&[("name", name)])
      .send()
      .await
      .unwrap()
      .json::<HttpRes<Vec<ModelInner>>>()
      .await
      .unwrap();
  }

  async fn register(&mut self, model: Model) -> Result<()> {
    todo!()
  }

  async fn deregister(&mut self, model: Model) -> Result<()> {
    todo!()
  }
}

pub struct RemoteRegistry<T> {
  client: T,
}

impl<T: RemoteClient> RemoteRegistry<T> {
  pub fn new(client: T) -> Self {
    Self { client }
  }
}

#[async_trait]
impl<T: RemoteClient + Send + Sync + 'static> Registry for RemoteRegistry<T> {
  async fn search(&self, name: &str) -> Result<Vec<Model>> {
    self.client.search(name).await
  }

  async fn register(&mut self, model: Model) -> Result<()> {
    self.client.register(model).await
  }

  async fn deregister(&mut self, model: Model) -> Result<()> {
    self.client.deregister(model).await
  }
}
