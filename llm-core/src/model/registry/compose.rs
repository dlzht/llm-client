use async_trait::async_trait;

use crate::model::{Model, registry::registry::Registry};

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
  async fn search(&self, name: &str) -> crate::errors::Result<Vec<Model>> {
    // let models = futures::stream::iter(self.registries.iter())
    //   .map(|r| r.search(name))
    //   .buffer_unordered(10)
    //   .filter(|r| async { r.is_ok() })
    //   .flatten()
    //   .collect::<Vec<_>>();
    // Ok(models)
    todo!()
  }

  async fn register(&mut self, model: Model) -> crate::errors::Result<()> {
    todo!()
  }

  async fn deregister(&mut self, model: Model) -> crate::errors::Result<()> {
    // let _ = futures::stream::iter(self.registries.iter())
    //   .for_each(|mut r| r.deregister(model));
    Ok(())
  }
}
