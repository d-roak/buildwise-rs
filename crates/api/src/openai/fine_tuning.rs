use anyhow::Result;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::fine_tuning::{
    CreateFineTuningJobRequest,
    ListFineTuningJobEventsRequest,
    RetrieveFineTuningJobRequest,
    CancelFineTuningJobRequest,
};
use openai_api_rs::v1::common::GPT3_5_TURBO_1106;
use std::env;

pub fn run() -> Result<String> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    // TODO:
    // - add files before using them
    // - fine tune on top of previous fine tuned models
    let req = CreateFineTuningJobRequest::new(
        GPT3_5_TURBO_1106.to_string(),
        "file-UtyRJcl153vtZxZT5aJmXVFj".into()
    );

    let result = client.create_fine_tuning_job(req)?;
    println!("{:?}", result);

    Ok("hello".into())
}
