use anyhow::{Result, Context};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::fine_tuning::{self,
    CreateFineTuningJobRequest,
    ListFineTuningJobEventsRequest,
    RetrieveFineTuningJobRequest,
    CancelFineTuningJobRequest,
};
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT3_5_TURBO_1106;
use std::env;

pub fn run() -> Result<String> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = CreateFineTuningJobRequest::new(
        GPT3_5_TURBO_1106.to_string(),
        "file-UtyRJcl153vtZxZT5aJmXVFj".into()
    );

    let result = client.create_fine_tuning_job(req)?;
    println!("{:?}", result);

    let res2 = client.list_fine_tuning_jobs()?;
    println!("{:?}", res2);


    let list_req = ListFineTuningJobEventsRequest::new(
        "ftjob-V4xY1YxcfsybbmUccGg57C9V".into()
    );
    let res3 = client.list_fine_tuning_job_events(list_req)?;
    println!("{:?}", res3);

    let ret_req = RetrieveFineTuningJobRequest::new(
        "ftjob-V4xY1YxcfsybbmUccGg57C9V".into()
    );
    let res4 = client.retrieve_fine_tuning_job(ret_req)?;
    println!("{:?}", res4);

    let cancel_req = CancelFineTuningJobRequest::new(
        "ftjob-V4xY1YxcfsybbmUccGg57C9V".into()
    );
    let res5 = client.cancel_fine_tuning_job(cancel_req)?;
    println!("{:?}", res5);

    Ok("hello".into())
}
