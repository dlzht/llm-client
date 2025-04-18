use async_trait::async_trait;

use crate::{errors::Result, model::Model};

#[async_trait]
pub trait Registry {
  async fn search(&self, name: &str) -> Result<Vec<Model>>;

  async fn register(&mut self, model: Model) -> Result<()>;

  async fn deregister(&mut self, model: Model) -> Result<()>;
}
