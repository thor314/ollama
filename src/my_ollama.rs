// see api: https://github.com/ollama/ollama/blob/main/docs/api.md
// docs: https://github.com/pepperoni21/ollama-rs
// bare: https://docs.rs/ollama-rs/latest/ollama_rs/generation/index.html
// chatbot: https://github.com/pepperoni21/ollama-rs/blob/master/examples/basic_chatbot.rs

use std::fmt::Display;

use anyhow::Context;
use futures_util::StreamExt;
use log::info;
use ollama_rs::{
  generation::completion::{
    request::GenerationRequest, GenerationContext, GenerationResponseStream,
  },
  models::create::CreateModelRequest,
  Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};

use crate::{error::MyError, MyResult};

const MODEL: &str = "llama2:latest";

pub async fn stream_single_prompt(
  prompt: impl Into<String> + Display + Clone,
) -> Result<(), MyError> {
  let ollama = Ollama::default();
  let model = MODEL.into();
  let mut stream =
    ollama.generate_stream(GenerationRequest::new(model, prompt.clone().into())).await.unwrap();
  info!("streaming prompt: {prompt}");
  let mut stdout = tokio::io::stdout();
  while let Some(res) = stream.next().await {
    let res = res.unwrap(); //.context("model generation failed")?;
    let res = &res[0].response; // contains an array of the next word
    stdout.write_all(res.as_bytes()).await?;
    stdout.flush().await?; // uncomment to stream word-by-word, rather than line-by-line
  }
  Ok(())
}

pub async fn single_prompt(prompt: &str) -> Result<String, MyError> {
  let ollama = Ollama::default();
  let model = MODEL.into();
  let res = ollama
    .generate(GenerationRequest::new(model, prompt.into()))
    .await
    .expect("model generation failed")
    .response;

  info!("{prompt}:\n{res}");
  Ok(res)
}

// create a non-streaming model
pub async fn create_model() {
  let ollama = Ollama::default();
  let res = ollama
    .create_model(CreateModelRequest::path("{MODEL}".into(), "/tmp/Modelfile.example".into()))
    .await
    .unwrap();
}

pub async fn create_chatbot() -> MyResult<()> {
  let ollama = Ollama::default();
  let mut stdout = stdout();
  // handles prior conversation context
  let mut context: Option<GenerationContext> = None;

  loop {
    // Prompt for input
    stdout.write_all(b"\nollama> ").await?;
    stdout.flush().await?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim_end();
    if input.eq_ignore_ascii_case("exit") {
      break;
    }

    // handle input, including prior conversation context
    let mut request = GenerationRequest::new(MODEL.into(), input.to_string());
    if let Some(context) = context.clone() {
      request = request.context(context);
    }
    let mut stream: GenerationResponseStream = ollama.generate_stream(request).await.unwrap();

    while let Some(Ok(res)) = stream.next().await {
      for ele in res {
        stdout.write_all(ele.response.as_bytes()).await?;
        stdout.flush().await?;

        if let Some(final_data) = ele.final_data {
          context = Some(final_data.context);
        }
      }
    }
  }

  Ok(())
}
