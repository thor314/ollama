#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;
use log::{error, info};
use ollama_rs::generation::completion::request::GenerationRequest;

mod error;
mod my_ollama;
#[cfg(test)] mod tests;
mod utils;

type MyResult<T> = Result<T, MyError>;

#[tokio::main]
async fn main() -> MyResult<()> {
  utils::setup()?;
  let default_prompt = "say hi".to_string();
  let prompt = std::env::args().nth(1).unwrap_or(default_prompt);
  // my_ollama::single_prompt(&prompt).await?;
  // my_ollama::stream_single_prompt(prompt, None).await?;
  my_ollama::create_chatbot().await?;

  Ok(())
}
