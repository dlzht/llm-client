use async_trait::async_trait;

use crate::model::{Model, ModelRef, registry::registry::Registry};

#[derive(Default)]
pub struct ComposedRegistry {
  registries: Vec<Box<dyn Registry + Send + Sync + 'static>>,
}

impl ComposedRegistry {
  pub fn new(
    registries: impl IntoIterator<Item = Box<dyn Registry + Send + Sync + 'static>>,
  ) -> Self {
    ComposedRegistry {
      registries: registries.into_iter().collect(),
    }
  }

  pub fn add_registry(&mut self, registry: impl Registry + Send + Sync + 'static) {
    let registry = Box::new(registry) as Box<dyn Registry + Send + Sync + 'static>;
    self.registries.push(registry);
  }
}

#[async_trait]
impl Registry for ComposedRegistry {
  async fn search(&self, name: &str) -> crate::errors::Result<Vec<ModelRef>> {
    todo!()
  }

  async fn register(&mut self, model: Model) -> crate::errors::Result<()> {
    todo!()
  }

  async fn deregister(&mut self, model: ModelRef) -> crate::errors::Result<()> {
    Ok(())
  }
}
