use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_1106_PREVIEW;

async fn root() -> String {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    println!("{}", env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = ChatCompletionRequest::new(
        GPT4_1106_PREVIEW.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("Hello OpenAI!")),
            name: None,
        }],
    );

    let result = client.chat_completion(req).unwrap();
    println!("{:?}", result.choices[0].message.content);

    "Hello, World!".into()
}
