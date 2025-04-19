use reqwest::Client;
use serde_json::json;
use std::error::Error;

use crate::models::payload::Payload;

pub struct Send;

impl Send {
    pub async fn send_speech(input: Payload, language: String) -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        let lang_payload = json!({ "language": language });
        let speech_payload = input.to_json()?;

        client
            .patch("http://127.0.0.1:8000/set_parameter")
            .json(&lang_payload)
            .send()
            .await?;

        client
            .post("http://127.0.0.1:8000/to_speech")
            .json(&speech_payload)
            .send()
            .await?;

        Ok(())
    }
}
