use serde::{Deserialize, Serialize};
use crate::message::Message;

#[derive(Debug, Clone, Serialize)]
pub struct Request<'a> {
   pub model: &'a str,
   pub messages: &'a [Message],
   pub stream: Option<bool>,
   pub temperature: Option<f32>,
   pub max_tokens: Option<i32>,
   pub n: Option<i32>,
   pub seed: Option<i32>,
   pub stop: Option<&'a Vec<String>>
}

impl<'a> Request<'a> {
   pub fn new(model: &'a str, messages: &'a [Message]) -> Self {
      Request {
         model,
         messages,
         stream: None,
         temperature: None,
         max_tokens: None,
         n: None,
         seed: None,
         stop: None,
      }
   }
}