use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::{Client as ReqwestClient, RequestBuilder};
use serde::Deserialize;
use snafu::{OptionExt, ResultExt};

use crate::{
  errors::{ImpossibleSnafu, PlainMessageSnafu, ReqwestClientSnafu, Result},
  model::{Model, ModelRef, registry::registry::Registry},
};

#[async_trait]
pub trait RegistryClient {
  async fn search(&self, name: &str) -> Result<Vec<ModelRef>>;

  async fn register(&mut self, model: Model) -> Result<()>;

  async fn deregister(&mut self, model: ModelRef) -> Result<()>;
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
pub struct HttpRes<T> {
  code: i32,
  message: Option<String>,
  data: Option<T>,
}

impl<T> HttpRes<T> {
  pub fn unwrap_data(self) -> Result<Option<T>> {
    if self.code != 0 {
      let message = format!(
        "Failed to unwrap http response: {} {}",
        self.code,
        self.message.unwrap_or("None".to_string())
      );
      return Err(PlainMessageSnafu { message }.build());
    }
    Ok(self.data)
  }
}

#[async_trait]
impl RegistryClient for HttpClient {
  async fn search(&self, name: &str) -> Result<Vec<ModelRef>> {
    let req = self.search.try_clone().context(ImpossibleSnafu)?;
    let res = req
      .query(&[("name", name)])
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<HttpRes<Vec<Model>>>()
      .await
      .context(ReqwestClientSnafu)?
      .unwrap_data()?
      .map(|ms| ms.into_iter().map(|m| m.into()).collect())
      .unwrap_or_else(Vec::new);
    Ok(res)
  }

  async fn register(&mut self, model: Model) -> Result<()> {
    let req = self.register.try_clone().context(ImpossibleSnafu)?;
    let _res = req
      .json(&model)
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<HttpRes<()>>()
      .await
      .context(ReqwestClientSnafu)?
      .unwrap_data()?;
    Ok(())
  }

  async fn deregister(&mut self, model: ModelRef) -> Result<()> {
    let req = self.deregister.try_clone().context(ImpossibleSnafu)?;
    let _res = req
      .json(model.as_ref())
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<HttpRes<()>>()
      .await
      .context(ReqwestClientSnafu)?
      .unwrap_data()?;
    Ok(())
  }
}

pub struct RemoteRegistry<T> {
  client: T,
}

impl<T: RegistryClient> RemoteRegistry<T> {
  pub fn new(client: T) -> Self {
    Self { client }
  }
}

#[async_trait]
impl<T: RegistryClient + Send + Sync + 'static> Registry for RemoteRegistry<T> {
  async fn search(&self, name: &str) -> Result<Vec<ModelRef>> {
    self.client.search(name).await
  }

  async fn register(&mut self, model: Model) -> Result<()> {
    self.client.register(model).await
  }

  async fn deregister(&mut self, model: ModelRef) -> Result<()> {
    self.client.deregister(model).await
  }
}
