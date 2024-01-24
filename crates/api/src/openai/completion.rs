use anyhow::{Result, Context};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use std::env;

pub fn query(_model: String, input: String) -> Result<String> {
    let client = Client::new(env::var("OPENAI_API_KEY")?.to_string());
    let req = ChatCompletionRequest::new(
        // TODO: use dynamic model with fine tuned data
        env::var("OPENAI_MODEL")?,
        vec![
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(
                    "Buildwise is a chatbot that gives suggestions about gas optimizations for solidity.".into()
                ),
                name: None,
            },
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(input),
                name: None,
            }
        ],
    );

    let result = client.chat_completion(req)?;
    result.choices[0].message.content.clone().context("No content")
}
