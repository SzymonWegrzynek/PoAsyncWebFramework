#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use serde_json::{Value, json};
    use std::error::Error;
    use tokio;

    pub async fn send_payload(payload: Value) -> Result<StatusCode, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .post("http://localhost:8000/to_speech")
            .json(&payload)
            .send()
            .await?;

        Ok(resp.status())
    }

    #[tokio::test]
    async fn test_send_valid_text() {
        let payload = json!({
            "type": "text",
            "data": {
                "text": "stół bez nóg"
            }
        });

        let status = send_payload(payload).await.unwrap();
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_server_error_response() {
        let long_text = "maciek".repeat(1000);

        let payload = json!({
            "type": "text",
            "data": {
                "text": long_text
            }
        });

        let status = send_payload(payload).await.unwrap();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
