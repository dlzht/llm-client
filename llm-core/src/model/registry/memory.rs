use std::sync::Arc;
use async_trait::async_trait;
use dashmap::DashMap;

use crate::{
  errors::Result,
  model::{ModelRef, registry::registry::Registry},
};
use crate::model::Model;

pub struct MemoryRegistry {
  models: DashMap<String, ModelRef>,
}

impl MemoryRegistry {}

#[async_trait]
impl Registry for MemoryRegistry {
  async fn search(&self, name: &str) -> Result<Vec<ModelRef>> {
    let models = self
      .models
      .iter()
      .filter(|m| m.real_name() == name)
      .map(|m| m.clone())
      .collect::<Vec<_>>();
    Ok(models)
  }

  async fn register(&mut self, model: Model) -> Result<()> {
    let model = Arc::new(model);
    self.models.insert(model_key(&model), model);
    Ok(())
  }

  async fn deregister(&mut self, model: ModelRef) -> Result<()> {
    self.models.remove(&model_key(&model));
    Ok(())
  }
}

fn model_key(model: &ModelRef) -> String {
  format!("{}", model.real_name())
}
