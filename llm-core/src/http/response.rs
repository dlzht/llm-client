use serde::Deserialize;

use crate::errors::{PlainMessageSnafu, Result};

const SUCCESS_CODE: i32 = 0;

#[derive(Debug, Deserialize)]
pub struct HttpRes<T> {
  code: i32,
  message: Option<String>,
  data: Option<T>,
}

impl<T> HttpRes<T> {
  pub fn is_success(&self) -> bool {
    self.code == SUCCESS_CODE
  }

  pub fn has_data(&self) -> bool {
    self.code == SUCCESS_CODE && self.data.is_some()
  }

  pub fn unwrap_data(self) -> Result<Option<T>> {
    if self.code != SUCCESS_CODE {
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
