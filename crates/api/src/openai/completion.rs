use anyhow::{Result, Context};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT3_5_TURBO_1106;
use std::env;

pub fn query(_model: String, input: String) -> Result<String> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO_1106.to_string(),
        vec![
            chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(
                    "Given the following code, improve (if possible) for gas effectiveness or security, in GitHub suggestion format (starting with '```suggestion'). The suggestion must be code changes and don't output any other text. If there is no 10x suggestion, return None".into()
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
    println!("{:?}", result);
    result.choices[0].message.content.clone().context("No content")
}
