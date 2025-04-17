use reqwest::{Method, RequestBuilder};
use crate::message::{Message, Messages};
use crate::model::Model;
use crate::request::Request;
use crate::response::Response;

#[derive(Debug)]
pub struct Session {
    model: Model,
    messages: Messages,
    request: RequestBuilder
}

impl Session {
    pub fn new(model: Model, request: RequestBuilder) -> Self {
        Session {
            model,
            messages: Messages::new(),
            request
        }
    }

    pub fn system_default(&mut self) {
        let message = Message::system("You are a helpful assistant.");
        self.messages.push(message);
    }

    pub fn system_message(&mut self, message: impl Into<String>) {
        let message = Message::system(message);
        self.messages.push(message);
    }

    pub async fn ask_question(&mut self, question: impl Into<String>) -> Response {
        let message = Message::user(question);
        self.messages.push(message);
        let payload = self.create_payload();
        let request = self.request.try_clone().unwrap();
        let res = request.json(&payload).send().await.unwrap()
            .json::<Response>().await.unwrap();
        res
    }

    fn create_payload(&self) -> Request {
        let mut request = Request::new(
            self.model.real_name(),
            self.messages.message_ref()
        );
        request.max_tokens = Some(10);
        request.stream = Some(false);
        request
    }
}