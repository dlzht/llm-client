use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{
  errors::{DeserializeJsonSnafu, DeserializeTOMLSnafu, Result},
  model::Model,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NestModels {
  models: Vec<Model>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Models {
  Nest(NestModels),
  Array(Vec<Model>),
}

impl Models {
  fn models(self) -> Vec<Model> {
    match self {
      Models::Nest(m) => m.models,
      Models::Array(m) => m,
    }
  }
}

pub struct ModelParse;

impl ModelParse {
  pub fn model_from_json(json: impl AsRef<str>) -> Result<Model> {
    let model = serde_json::from_str::<Model>(json.as_ref()).context(DeserializeJsonSnafu)?;
    Ok(model)
  }

  pub fn model_from_toml(toml: impl AsRef<str>) -> Result<Model> {
    let model = toml::from_str::<Model>(toml.as_ref()).context(DeserializeTOMLSnafu)?;
    Ok(model)
  }

  pub fn models_from_json(json: impl AsRef<str>) -> Result<Vec<Model>> {
    let models = serde_json::from_str::<Models>(json.as_ref()).context(DeserializeJsonSnafu)?;
    Ok(models.models())
  }

  pub fn models_from_toml(toml: impl AsRef<str>) -> Result<Vec<Model>> {
    let models = toml::from_str::<Models>(toml.as_ref()).context(DeserializeTOMLSnafu)?;
    Ok(models.models())
  }
}

#[cfg(test)]
mod test {
  use crate::model::parse::ModelParse;

  #[test]
  fn test_model_parse() {
    let json = "{\"real_name\": \"model_01\", \"api_endpoint\": \"https://endpoint.com/model_01\"}";
    let model = ModelParse::model_from_json(json);
    assert!(model.is_ok());

    let toml = "real_name = \"model_01\"\napi_endpoint = \"https://endpoint.com/model_01\"";
    let model = ModelParse::model_from_toml(toml);
    assert!(model.is_ok());
  }

  #[test]
  fn test_models_parse() {
    let json =
      "[{\"real_name\": \"model_01\", \"api_endpoint\": \"https://endpoint.com/model_01\"}]";
    let models = ModelParse::models_from_json(json);
    assert!(models.is_ok());

    let json = "{\"models\": [{\"real_name\": \"model_01\", \"api_endpoint\": \"https://endpoint.com/model_01\"}]}";
    let models = ModelParse::models_from_json(json);
    assert!(models.is_ok());

    let toml =
      "[[models]]\nreal_name = \"model_01\"\napi_endpoint = \"https://endpoint.com/model_01\"";
    let models = ModelParse::models_from_toml(toml);
    assert!(models.is_ok());
  }
}
