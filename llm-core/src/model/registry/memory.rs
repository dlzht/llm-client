use async_trait::async_trait;
use dashmap::DashMap;

use crate::{
  errors::Result,
  model::{Model, registry::registry::Registry},
};

pub struct MemoryRegistry {
  models: DashMap<String, Model>,
}

impl MemoryRegistry {}

#[async_trait]
impl Registry for MemoryRegistry {
  async fn search(&self, name: &str) -> Result<Vec<Model>> {
    let models = self
      .models
      .iter()
      .filter(|m| m.real_name() == name)
      .map(|m| m.clone())
      .collect::<Vec<_>>();
    Ok(models)
  }

  async fn register(&mut self, model: Model) -> Result<()> {
    self.models.insert(model_key(&model), model);
    Ok(())
  }

  async fn deregister(&mut self, model: Model) -> Result<()> {
    self.models.remove(&model_key(&model));
    Ok(())
  }
}

fn model_key(model: &Model) -> String {
  format!("{}", model.real_name())
}
