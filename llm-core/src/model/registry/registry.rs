use async_trait::async_trait;

use crate::{
  errors::Result,
  model::{Model, ModelRef},
};

#[async_trait]
pub trait Registry {
  async fn search(&self, name: &str) -> Result<Vec<ModelRef>>;

  async fn register(&mut self, model: Model) -> Result<()>;

  async fn deregister(&mut self, model: ModelRef) -> Result<()>;
}
