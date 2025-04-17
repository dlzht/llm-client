use llm_core::{
  client::{DefaultClient, DefaultClientOptions},
  model::DefaultModel,
  session::DefaultSessionOptions,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let model = DefaultModel::new(
    "qwen-turbo",
    "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions",
  );
  let options = DefaultClientOptions::new("sk-c44f4293b3914363b4c1575019f0900b");
  let client = DefaultClient::new(options);
  let session_options = DefaultSessionOptions::new(model);
  let mut session = client.new_session(session_options).unwrap();
  session.play_as_assistant(true);
  let res = session.ask_question("苹果的营养高吗?").await;
  println!("{:?}", res);
}
