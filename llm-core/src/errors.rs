use snafu::{Location, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
  #[snafu(display("Failed to process http request"))]
  ReqwestClient {
    #[snafu(source)]
    source: reqwest::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Failed to process eventsource"))]
  Eventsource {
    #[snafu(source)]
    source: reqwest_eventsource::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Failed to serialize JSON"))]
  SerializeJson {
    #[snafu(source)]
    source: serde_json::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Failed to deserialize JSON"))]
  DeserializeJson {
    #[snafu(source)]
    source: serde_json::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Failed to serialize TOML"))]
  SerializeTOML {
    #[snafu(source)]
    source: toml::ser::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Failed to deserialize TOML"))]
  DeserializeTOML {
    #[snafu(source)]
    source: toml::de::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("{}", message))]
  PlainMessage {
    message: String,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Impossible error!"))]
  Impossible {
    #[snafu(implicit)]
    location: Location,
  },
}
